extern crate rand;
use std::io; 
use std::cmp::Ordering;  
use rand::Rng;

fn main()
{
    println!("=========WELLCOME IN GUESSING GAME =======");
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1,101);

    loop
    {
        println!("\n Please input a number ! ");

        let mut guess = String::new();


        io::stdin().read_line(&mut guess)
        .expect("failed to read the line");

        let guess: u32 = match guess.trim().parse()
        {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}",guess);

        match guess.cmp(&secret_number)
        {
            Ordering::Less => println!("\nTo small !"),
            Ordering::Greater => println!("\nTo big !"),
            Ordering::Equal => 
            {
                println!("\n==YOU WIN THE GAME  !");
                break;
            }
        }
        
    }
    println!("Do  you want to play again !");   

}
