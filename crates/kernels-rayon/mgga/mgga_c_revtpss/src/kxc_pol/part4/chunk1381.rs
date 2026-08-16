//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1381/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1381(t140: f64, t3698: f64, t5047: f64, t1222: f64, t1012: f64, t13026: f64, t16715: f64, t16720: f64, t5312: f64, t1774: f64, t3601: f64, t3611: f64) -> (f64, f64, f64, f64, f64) {
    let t17471 = t140 * t3698;
    let t17472 = t17471 * t5047;
    let t17474 = t1222 * t17472 / 324.0_f64;
    let t17475 = t1012 * t13026;
    let t17476 = t17475 * t16715;
    let t17479 = t5312 * t16720;
    let t17482 = t1774 * t3601;
    let t17483 = t17482 * t3611;
    (t17474, t17476, t17479, t17482, t17483)
}
