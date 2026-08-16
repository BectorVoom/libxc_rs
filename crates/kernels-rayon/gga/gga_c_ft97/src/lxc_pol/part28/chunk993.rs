//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 993/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk993(t7397: f64, t8232: f64, t33193: f64, t8392: f64, t604: f64, t7339: f64, t139320: f64, t139323: f64, t139492: f64, t139495: f64, t139533: f64, t1882: f64, t33077: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t139896 = 8.0_f64 / 27.0_f64 * t8232 * t7397;
    let t139940 = t8392 * t33193;
    let t139950 = t604 * t7339;
    let t139991 = 4.0_f64 / 9.0_f64 * t139320;
    let t139992 = 2.0_f64 / 9.0_f64 * t139323;
    let t140041 = 4.0_f64 / 9.0_f64 * t139492;
    let t140042 = 8.0_f64 / 9.0_f64 * t139495;
    let t140053 = 10.0_f64 / 9.0_f64 * t139533;
    let t140068 = t1882 * t33077;
    (t139896, t139940, t139950, t139991, t139992, t140041, t140042, t140053, t140068)
}
