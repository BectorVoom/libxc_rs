//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1230/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1230(t22296: f64, t22300: f64, t22344: f64, t22621: f64, t22623: f64, t22625: f64, t22627: f64, t56013: f64, t56014: f64, t56015: f64, t56016: f64, t56039: f64, t56040: f64) -> f64 {
    let t56259 = -t56013 + t56014 + t56015 - t56016 + t22296 - t22300 + t22344 + t22621 - t22623 + t22625 + t22627 - t56039 - t56040;
    t56259
}
