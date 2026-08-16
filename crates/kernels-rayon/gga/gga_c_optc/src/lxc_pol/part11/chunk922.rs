//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 922/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk922(t10188: f64, t13699: f64, t13701: f64, t13703: f64, t16630: f64, t16634: f64, t16638: f64, t16642: f64, t16646: f64, t16756: f64, t16759: f64, t17299: f64) -> f64 {
    let t17311 = 0.48461111111111111112e3_f64 * t13699 - 0.14538333333333333333e4_f64 * t13701 + 0.72691666666666666668e3_f64 * t13703 - 0.96922222222222222223e3_f64 * t10188 + 0.29076666666666666666e4_f64 * t16634 - 0.14538333333333333333e4_f64 * t16638 - 0.43614999999999999999e4_f64 * t16642 + 0.43614999999999999999e4_f64 * t16646 - 0.80768518518518518518e3_f64 * t16630 - 0.34962962962962962963e2_f64 * t16756 - 0.78666666666666666667e2_f64 * t16759;
    let t17312 = t17299 + t17311;
    t17312
}
