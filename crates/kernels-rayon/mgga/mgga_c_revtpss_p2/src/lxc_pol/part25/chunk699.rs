//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 699/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk699(t3088: f64, t4890: f64, t3299: f64, t1043: f64, t3154: f64, t3317: f64, t357: f64, t999: f64, t1012: f64, t1014: f64, t3252: f64, t354: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4891 = t3088 * t4890;
    let t4892 = t3299 * t4891;
    let t4894 = t3154 * t1043;
    let t4899 = t3317 * t4891;
    let t4900 = t1043 * t357;
    let t4910 = t357 * t999;
    let t4915 = t1012 * t1014;
    let t4919 = t1012 * t3252;
    let t4975 = t354 * t357;
    (t4891, t4892, t4894, t4899, t4900, t4910, t4915, t4919, t4975)
}
