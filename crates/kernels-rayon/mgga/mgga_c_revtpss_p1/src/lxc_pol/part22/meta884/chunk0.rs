//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3058/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3058(t2439: f64, t4622: f64, t15186: f64, t698: f64, t15177: f64, t15180: f64, t15162: f64, t15165: f64, t123: f64, t127: f64, t159: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t51915 = t2439 * t4622;
    let t51917 = t698 * t15186;
    let t51921 = t698 * t15177;
    let t51923 = t698 * t15180;
    let t51937 = t698 * t15162;
    let t51942 = t698 * t15165;
    let t51957 = t123 * t127 * t159;
    (t51915, t51917, t51921, t51923, t51937, t51942, t51957)
}
