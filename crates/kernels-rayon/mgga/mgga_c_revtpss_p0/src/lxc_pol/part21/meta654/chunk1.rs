//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2442/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2442(t11277: f64, t11916: f64, t11246: f64, t11251: f64, t3172: f64, t11648: f64, t3169: f64, t1062: f64, t11782: f64, t10356: f64, t11150: f64, t357: f64) -> (f64, f64, f64, f64, f64) {
    let t42374 = t11277 * t11916;
    let t42377 = t11246 * t3172 * t11251;
    let t42383 = t3169 * t11648;
    let t42391 = t11782 * t1062;
    let t42397 = t357 * t11150 * t10356;
    (t42374, t42377, t42383, t42391, t42397)
}
