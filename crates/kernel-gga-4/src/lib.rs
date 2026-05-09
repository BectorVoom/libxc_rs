#![allow(clippy::excessive_precision)]
#![allow(clippy::needless_late_init)]
#![allow(clippy::too_many_arguments)]

//! GGA kernel translations batch 4.

pub mod gga_x_beefvdw;

// Deferred: CubeCL proc macro SIGSEGV on large lxc_pol/kxc_pol files.
// pub mod gga_c_optc;
// pub mod gga_x_wpbeh;
