//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 677/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk677(t3196: f64, t599: f64, t1181: f64, t7337: f64, t3176: f64, t604: f64, t2068: f64, t1160: f64, t2067: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7338 = t599 * t3196;
    let t7339 = t1181 * t7338;
    let t7340 = t7337 * t7339;
    let t7342 = t604 * t3176;
    let t7343 = t1181 * t7342;
    let t7344 = t2068 * t7343;
    let t7346 = t1160 * t2067;
    (t7338, t7339, t7340, t7342, t7343, t7344, t7346)
}
