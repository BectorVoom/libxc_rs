//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 994/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk994(t1163: f64, t1165: f64, t1532: f64, t16548: f64, t3372: f64, t4959: f64, t1181: f64, t535: f64, t864: f64, t944: f64, t406: f64, t12801: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16551 = t1163 * t1165 * t1532 * t16548;
    let t16553 = t3372 * t4959;
    let t16557 = t1163 * t1181 * t535 * t16548;
    let t16559 = t944 * t864;
    let t16560 = t16559 * t406;
    let t16563 = t12801 * t1165 * t1532 * t16560;
    (t16551, t16553, t16557, t16559, t16560, t16563)
}
