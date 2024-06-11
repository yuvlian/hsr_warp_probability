use rand::Rng;
use std::io;
use std::sync::LazyLock;
use std::time::Instant;

static FIVE_STAR_CHARACTER_CHANCE: LazyLock<f64> = LazyLock::new(|| 0.006);
static FIVE_STAR_CONE_CHANCE: LazyLock<f64> = LazyLock::new(|| 0.008);
static CHARACTER_SOFT_PITY: LazyLock<u32> = LazyLock::new(|| 74);
static CONE_SOFT_PITY: LazyLock<u32> = LazyLock::new(|| 64);
static CHARACTER_PITY: LazyLock<u32> = LazyLock::new(|| 90);
static CONE_PITY: LazyLock<u32> = LazyLock::new(|| 80);
static LIMITED_CONE_CHANCE: LazyLock<f64> = LazyLock::new(|| 0.75);
static LIMITED_CHARACTER_CHANCE: LazyLock<f64> = LazyLock::new(|| 0.5);
static SOFT_PITY_INCREMENT: LazyLock<f64> = LazyLock::new(|| 0.06);

fn calculate_warp_probability(
    warps: u32,
    character_pity: u32,
    cone_pity: u32,
    cone_guaranteed: bool,
    character_guaranteed: bool,
    character_copies: u32,
    cone_copies: u32,
    num_simulations: u32,
) -> f64 {
    let mut successful_simulations = 0;

    for _ in 0..num_simulations {
        let mut warps_left = warps;
        let mut char_successes = 0;
        let mut cone_successes = 0;
        let mut curr_cone_pity = cone_pity;
        let mut curr_char_pity = character_pity;
        let mut curr_cone_guaranteed = cone_guaranteed;
        let mut curr_character_guaranteed = character_guaranteed;

        while warps_left > 0 {
            let random_value = rand::thread_rng().gen::<f64>();
            let mut curr_five_star_chance = *FIVE_STAR_CHARACTER_CHANCE;

            if cone_copies > 0 && char_successes < character_copies {
                curr_five_star_chance += *SOFT_PITY_INCREMENT
                    * f64::max(curr_char_pity as f64 - *CHARACTER_SOFT_PITY as f64, 0.0);

                if random_value < curr_five_star_chance || curr_char_pity + 1 == *CHARACTER_PITY {
                    if curr_character_guaranteed
                        || rand::thread_rng().gen::<f64>() < *LIMITED_CHARACTER_CHANCE
                    {
                        char_successes += 1;
                        curr_character_guaranteed = false;
                        curr_char_pity = 0;
                    } else {
                        curr_char_pity = 0;
                        curr_character_guaranteed = true;
                    }
                } else {
                    curr_char_pity += 1;
                }
            } else {
                curr_five_star_chance = *FIVE_STAR_CONE_CHANCE;
                curr_five_star_chance += *SOFT_PITY_INCREMENT
                    * f64::max(curr_cone_pity as f64 - *CONE_SOFT_PITY as f64, 0.0);

                if random_value < curr_five_star_chance || curr_cone_pity + 1 == *CONE_PITY {
                    if curr_cone_guaranteed
                        || rand::thread_rng().gen::<f64>() < *LIMITED_CONE_CHANCE
                    {
                        cone_successes += 1;
                        curr_cone_guaranteed = false;
                        curr_cone_pity = 0;
                    } else {
                        curr_cone_pity = 0;
                        curr_cone_guaranteed = true;
                    }
                } else {
                    curr_cone_pity += 1;
                }
            }
            warps_left -= 1;
        }

        if char_successes >= character_copies && cone_successes >= cone_copies {
            successful_simulations += 1;
        }
    }

    successful_simulations as f64 / num_simulations as f64
}

fn main() {
    let mut input = String::new();

    println!("Enter number of pulls you got:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let warps: u32 = input.trim().parse().expect("Please enter a valid number");
    println!("{}", warps);

    println!("Enter character pity:");
    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let character_pity: u32 = input.trim().parse().expect("Please enter a valid number");
    println!("{}", character_pity);

    println!("Enter lightcone pity:");
    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let cone_pity: u32 = input.trim().parse().expect("Please enter a valid number");
    println!("{}", cone_pity);

    println!("Is lightcone guaranteed (true/false):");
    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let cone_guaranteed: bool = input.trim().parse().expect("Please enter true or false");
    println!("{}", cone_guaranteed);

    println!("Is character guaranteed (true/false):");
    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let character_guaranteed: bool = input.trim().parse().expect("Please enter true or false");
    println!("{}", character_guaranteed);

    println!("Enter number of character copies wanted (E6 = 7):");
    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let character_copies: u32 = input.trim().parse().expect("Please enter a valid number");
    println!("{}", character_copies);

    println!("Enter number of lightcone copies wanted (S5 = 5):");
    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let cone_copies: u32 = input.trim().parse().expect("Please enter a valid number");
    println!("{}", cone_copies);

    println!("Enter number of simulations (press enter for default value):");
    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let num_simulations: u32 = if input.trim().is_empty() {
        10000
    } else {
        input.trim().parse().expect("Please enter a valid number")
    };
    println!("{}", num_simulations);

    let start_time = Instant::now();
    let probability = calculate_warp_probability(
        warps,
        character_pity,
        cone_pity,
        cone_guaranteed,
        character_guaranteed,
        character_copies,
        cone_copies,
        num_simulations,
    );
    let elapsed_time = start_time.elapsed();

    println!("Estimated probability: {:.6}%", probability * 100.0);
    println!("Elapsed time: {:.2?}", elapsed_time);
}
