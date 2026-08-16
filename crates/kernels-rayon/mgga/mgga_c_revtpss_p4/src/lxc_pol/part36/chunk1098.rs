//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1098/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1098(t5486: f64, t6573: f64, t1287: f64, t1811: f64, t6622: f64, t13149: f64, t24911: f64, t6587: f64, t1280: f64, t24713: f64, t13129: f64, t1774: f64, t21541: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t24922 = t5486 * t6573;
    let t24928 = t1811 * t6622 * t1287;
    let t24931 = t24911 * t13149;
    let t24934 = t5486 * t6587;
    let t24941 = t1280 * t24713;
    let t24948 = t24911 * t13129;
    let t24951 = t21541 * t1774;
    (t24922, t24928, t24931, t24934, t24941, t24948, t24951)
}
