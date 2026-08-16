//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1050/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1050(t146: f64, t2145: f64, t3177: f64, t3090: f64, t560: f64, t3232: f64, t6897: f64, t2333: f64, t9563: f64, t19026: f64, t3245: f64, t1275: f64, t2924: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t31060 = t146 * t2145 * t3177;
    let t31064 = t3090 * t560;
    let t31393 = t3232 * t6897;
    let t31498 = t9563 * t2333;
    let t31510 = t3245 * t19026;
    let t31689 = t2924 * t1275;
    (t31060, t31064, t31393, t31498, t31510, t31689)
}
