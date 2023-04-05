#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod tictactoe {

    use ink::{
        prelude::vec::Vec,
    };

    use ink::prelude::string::String;

    use ink::storage::Mapping;
    use ink::prelude::string::ToString;

    use scale::{
        Decode,
        Encode,
    };

    #[derive(Encode, Decode, Debug, PartialEq, Eq, Copy, Clone)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        AlreadyStarted,
        NotStarted,
        NotEnoughMoney,
    }

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct TicTacToe {
        /// Stores a single `bool` value on the storage.
        map: Vec<Vec<u8>>,
        turn: u8,
        allowed_players: Vec<AccountId>,
        started: bool,
        player_number: Mapping<AccountId, u8>,
        terminated: bool,
        winner: AccountId,
    }

    impl TicTacToe {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut new_map = Vec::new();
            for _ in 0..5 {
                let mut row = Vec::new();
                for _ in 0..5 {
                    row.push(0);
                }
                new_map.push(row);
            }

            Self {
                map: new_map,
                turn: 0,
                allowed_players: Vec::new(),
                started: false,
                player_number: Mapping::new(),
                terminated: false,
                winner: AccountId::from([0x0; 32]),
            }

        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message, payable)]
        pub fn join(&mut self, player_number: u8)-> Result<(),Error > {
            if self.started{
                return Err(Error::AlreadyStarted);
            }
            if self.env().transferred_value() < 1000{
                return Err(Error::NotEnoughMoney)
            }

            self.allowed_players.push(self.env().caller());
            self.player_number.insert(self.env().caller(), &player_number);
            Ok(())
        }

        #[ink(message)]
        pub fn start(&mut self) -> Result<(),Error >{
            if self.allowed_players.len() >= 2 {
                self.started = true;
                Ok(())
            }
            else {
                return Err(Error::NotStarted);
            }
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn make_move(&mut self,coord_x: u32, coord_y: u32) -> bool {
            if self.started == false {
                false;
            }
            //self.map[coord_x as usize][coord_y as usize] = self.player_number.get(self.env().caller()).unwrap();
            let caller_number = self.player_number.get(self.env().caller()).unwrap();
            *self.map.get_mut(coord_x as usize).unwrap().get_mut(coord_y as usize).unwrap() = caller_number;

            true
        }

        #[ink(message)]
        pub fn get_map(&self) -> String {
            let mut output = String::from("Map: ");
            for row in self.map.iter() {
                for cell in row.iter() {
                    output = output + &cell.to_string();
                }
                output = output + " ";
            }
            output
        }

        #[ink(message)]
        pub fn has_won(&mut self)-> Result<AccountId,Error>{
            //Check whether someone has won tictactoe
            //Return the account id of the winner
            //If no one has won, return an error
            for players in self.allowed_players.iter(){
                let player_number = &self.player_number.get(players).unwrap();
                //Check rows
                for row in self.map.iter(){
                    let mut row_count = 0;
                    for cell in row.iter(){
                        if cell == player_number{
                            row_count += 1;
                        }
                    }
                    if row_count == 5{
                        self.terminated = true;
                        self.winner = *players;
                        return Ok(*players);
                    }
                }
                //Check columns
                for i in 0..5{
                    let mut col_count = 0;
                    for row in self.map.iter(){
                        if row[i] == *player_number{
                            col_count += 1;
                        }
                    }
                    if col_count == 5{
                        self.terminated = true;
                        self.winner = *players;
                        return Ok(*players);
                    }
                }
  
            }
            Err(Error::NotStarted)
        }
     
        #[ink(message)]
        pub fn get_winner(&self) -> AccountId {
            self.winner
        }

        #[ink(message)]
        pub fn claim_reward(&mut self) -> bool {
            if self.env().caller() == self.winner{
                self.env().transfer(self.env().caller(), self.env().balance());
                true
            }
            else{
                false
            }
        }

        #[ink(message)]
        pub fn get_number(&self) -> u8 {
            self.player_number.get(self.env().caller()).unwrap()
        }

    }



    
}
