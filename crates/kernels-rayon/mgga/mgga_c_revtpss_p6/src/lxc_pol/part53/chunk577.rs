//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 577/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk577(t3317: f64, t4891: f64, t1043: f64, t357: f64, t4893: f64, t3117: f64, t1651: f64, t1045: f64, t999: f64, t4781: f64, t1012: f64, t1014: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4899 = t3317 * t4891;
    let t4900 = t1043 * t357;
    let t4901 = t4893 * t4900;
    let t4902 = t3117 * t4901;
    let t4905 = t1651 * t1043;
    let t4906 = t4905 * t1045;
    let t4907 = t3117 * t4906;
    let t4910 = t357 * t999;
    let t4911 = t4781 * t4910;
    let t4912 = t3117 * t4911;
    let t4915 = t1012 * t1014;
    (t4899, t4902, t4905, t4907, t4912, t4915)
}
