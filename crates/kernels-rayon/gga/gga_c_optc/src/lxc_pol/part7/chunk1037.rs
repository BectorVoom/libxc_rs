//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1037/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1037(t22626: f64, t22290: f64, t22293: f64, t22296: f64, t22300: f64, t22303: f64, t22306: f64, t22340: f64, t22342: f64, t22344: f64, t22621: f64, t22623: f64, t22625: f64) -> (f64, f64) {
    let t22627 = 384.0_f64 * t22626;
    let t22628 = -t22290 + t22293 + t22296 - t22300 + t22303 + t22306 + t22340 + t22342 + t22344 + t22621 - t22623 + t22625 - t22627;
    (t22627, t22628)
}
