//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2946/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2946(t1857: f64, t9342: f64, t9855: f64, t9410: f64, t9413: f64, t5571: f64, t9372: f64, t13597: f64, t2496: f64, t123: f64, t2630: f64, t5566: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t48287 = t9342 * t1857;
    let t48290 = t9855 * t1857;
    let t48292 = t9410 * t1857;
    let t48294 = t9413 * t1857;
    let t48297 = t5571 * t9372;
    let t48299 = t13597 * t2496;
    let t48302 = t5566 * t123 * t2630;
    (t48287, t48290, t48292, t48294, t48297, t48299, t48302)
}
