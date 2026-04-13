#![allow(clippy::excessive_precision)]
#![allow(clippy::needless_late_init)]
#![allow(clippy::too_many_arguments)]

//! GGA kernel translations from maple2c.
//! Each functional gets a kernel file (e.g., gga_c_pbe.rs) and a launch wrapper (launch_gga_c_pbe.rs).

// Placeholder stub modules -- GGA kernel translations are added here in later plans.
pub mod order0;
pub mod order1;
pub mod order2;
pub mod order3;
pub mod order4;
