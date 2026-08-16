//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 156/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk156(t406: f64, t415: f64, t241: f64, t391: f64, t393: f64, t402: f64, rho1: f64) -> (f64, f64, f64) {
    let t416 = t406 * t415;
    let t419 = t241 * (-0.3109e-1_f64 * t393 * t402 + t391 - 0.19751789702565206229e-1_f64 * t416);
    let t421 = 0.19751789702565206229e-1_f64 * t241 * t416;
    let t422 = rho1 * rho1;
    (t419, t421, t422)
}
