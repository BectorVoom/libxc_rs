//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 875/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk875(t1342: f64, t4815: f64, t2373: f64, t10188: f64, t13699: f64, t13701: f64, t13703: f64, t16630: f64, t16634: f64, t16638: f64, t16642: f64, t16646: f64, t16650: f64, t7609: f64) -> (f64, f64, f64) {
    let t16655 = t4815 * t1342;
    let t16657 = 6.0_f64 * t2373 * t16655;
    let t16671 = -t7609 - 0.12361111111111111111e-1_f64 * t10188 + 0.61805555555555555556e-2_f64 * t13699 - 0.18541666666666666667e-1_f64 * t13701 + 0.92708333333333333334e-2_f64 * t13703 - 0.10300925925925925926e-1_f64 * t16630 + 0.37083333333333333333e-1_f64 * t16634 - 0.18541666666666666666e-1_f64 * t16638 - 0.55625000000000000001e-1_f64 * t16642 + 0.55625000000000000001e-1_f64 * t16646 - 0.92708333333333333333e-2_f64 * t16650;
    (t16655, t16657, t16671)
}
