//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 949/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk949(t3077: f64, t7646: f64, t1167: f64, t30861: f64, t7495: f64, t1181: f64, t15407: f64, t31562: f64, t599: f64, t7676: f64, t7720: f64, t2092: f64, t7630: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t31611 = t3077 * t7646;
    let t31612 = t31611 * t1167;
    let t31619 = t30861 * t7495;
    let t31623 = t31562 * t1181 * t599 * t15407;
    let t31625 = t7676 * t7720;
    let t31627 = t7630 * t2092;
    (t31611, t31612, t31619, t31623, t31625, t31627)
}
