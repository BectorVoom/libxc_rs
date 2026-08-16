//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 538/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk538(t3317: f64, t4891: f64, t1012: f64, t1014: f64, t3252: f64, t140: f64, t1655: f64, t1011: f64, t1678: f64, t342: f64, t1086: f64, t1647: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4899 = t3317 * t4891;
    let t4915 = t1012 * t1014;
    let t4919 = t1012 * t3252;
    let t4924 = t140 * t1655;
    let t4925 = t1011 * t4924;
    let t4935 = t342 * t1678;
    let t4954 = t1647 * t1086;
    (t4899, t4915, t4919, t4924, t4925, t4935, t4954)
}
