//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 923/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk923(t1165: f64, t30209: f64, t3044: f64, t604: f64, t2082: f64, t31289: f64, t2109: f64, t7780: f64, t1982: f64, t2015: f64, t14575: f64, t7346: f64) -> (f64, f64, f64, f64, f64) {
    let t31720 = t30209 * t1165 * t604 * t3044;
    let t31721 = 0.94344276868812456204e-3_f64 * t31720;
    let t31750 = t31289 * t2082;
    let t31751 = 0.13505315707191967146e-1_f64 * t31750;
    let t31752 = t7780 * t2109;
    let t31773 = t2015 * t1982;
    let t31797 = t7346 * t1165 * t604 * t14575;
    (t31721, t31751, t31752, t31773, t31797)
}
