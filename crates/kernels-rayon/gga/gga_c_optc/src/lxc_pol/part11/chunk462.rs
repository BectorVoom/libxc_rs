//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 462/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk462(t1198: f64, t484: f64, t481: f64, t2843: f64, t2865: f64, t474: f64, t1084: f64, t411: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2885 = 1.0_f64 / t1198 / t484;
    let t2886 = t481 * t2885;
    let t2890 = 0.96922222222222222222e3_f64 * t2843;
    let t2895 = 0.13111111111111111111e3_f64 * t2865;
    let t2910 = t474 * t474;
    let t2911 = 1.0_f64 / t2910;
    let t2915 = t1084 * t411;
    let t2916 = 1.0_f64 / t2915;
    (t2885, t2886, t2890, t2895, t2910, t2911, t2915, t2916)
}
