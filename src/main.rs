use std::io;
use std::cmp::Ordering;

fn main() {
    loop {
        let mut guess = String::new();

        println!("Please type a positive number (0 to quit): ");
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&0) {
            Ordering::Less => fizzbuzz(guess),
            Ordering::Greater => fizzbuzz(guess),
            Ordering::Equal => break
        }
    }
}

fn fizzbuzz(n: u32) {
    for x in 1..(n + 1) {
        let mod_three = x % 3 == 0;
        let mod_five  = x % 5 == 0;

        if mod_three && mod_five {
            println!("fizzbuzz");
        } else if mod_three {
            println!("fizz");
        } else if mod_five {
            println!("buzz");
        } else {
            println!("{}", x);
        }
    }
}

#[test]
fn test_fizzbuzz() {
    
}
