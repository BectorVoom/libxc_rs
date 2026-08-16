//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 920/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk920(t10348: f64, t13649: f64, t13651: f64, t13653: f64, t16650: f64, t16747: f64, t16750: f64, t16763: f64, t16766: f64, t8319: f64, t8321: f64, t10188: f64, t13699: f64, t13701: f64, t13703: f64, t16630: f64, t16634: f64, t16638: f64, t16642: f64, t16646: f64, t16756: f64, t16759: f64) -> (f64, f64) {
    let t17263 = 0.821e-3_f64 * t13649 - 0.4926e-2_f64 * t13651 + 0.2463e-2_f64 * t13653 - t8319 - 0.19388333333333333333e1_f64 * t16650 - t8321 - 0.7389e-2_f64 * t16747 + 0.7389e-2_f64 * t16763 + 0.2463e-2_f64 * t16750 - 0.12315e-2_f64 * t16766 - 0.4105e-2_f64 * t10348;
    let t17275 = 0.12925555555555555555e1_f64 * t13699 - 0.38776666666666666665e1_f64 * t13701 + 0.19388333333333333333e1_f64 * t13703 - 0.2585111111111111111e1_f64 * t10188 + 0.77553333333333333331e1_f64 * t16634 - 0.38776666666666666665e1_f64 * t16638 - 0.11633e2_f64 * t16642 + 0.11633e2_f64 * t16646 - 0.21542592592592592592e1_f64 * t16630 - 0.54733333333333333333e-3_f64 * t16756 - 0.12315e-2_f64 * t16759;
    (t17263, t17275)
}
