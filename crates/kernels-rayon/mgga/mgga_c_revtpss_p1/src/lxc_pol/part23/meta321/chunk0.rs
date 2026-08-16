//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1610/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1610(t13665: f64, t2630: f64, t1857: f64, t3860: f64, t3863: f64, t5566: f64, t749: f64, t512: f64, t9856: f64, t1468: f64, t9605: f64, t2: f64, t3874: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13666 = t13665 * t2630;
    let t13668 = t3860 * t1857;
    let t13670 = t3863 * t1857;
    let t13680 = t5566 * t749;
    let t13682 = 2.0_f64 * t512 * t13680;
    let t13683 = 48.0_f64 * t9856;
    let t13687 = t9605 * t1468;
    let t13690 = t3874 * t2;
    (t13666, t13668, t13670, t13680, t13682, t13683, t13687, t13690)
}
