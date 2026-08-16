//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 840/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk840(t16287: f64, t197: f64, t10048: f64, t13536: f64, t13538: f64, t13543: f64, t193: f64, t4752: f64, t6653: f64, t750: f64, t201: f64, t5: f64) -> (f64, f64, f64) {
    let t16288 = t197 * t16287;
    let t16292 = t6653 - 2200.0_f64 / 27.0_f64 * t10048 + 200.0_f64 / 9.0_f64 * t13536 + 200.0_f64 / 9.0_f64 * t13543 - 25.0_f64 / 3.0_f64 * t193 * t13538 * t4752 - 25.0_f64 / 9.0_f64 * t193 * t750 * t16288;
    let t16294 = t5 * t16292 * t201;
    (t16288, t16292, t16294)
}
