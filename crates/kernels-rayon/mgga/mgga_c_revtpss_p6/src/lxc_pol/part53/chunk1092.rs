//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1092/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1092(t31834: f64, t846: f64, t119808: f64, t7063: f64, t31801: f64, t2769: f64, t31777: f64, t786: f64, t122: f64, t2466: f64, t31780: f64, t240: f64, t822: f64, t843: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t119914 = t31834 * t846;
    let t119919 = t7063 * t119808;
    let t119920 = t119919 * t31801;
    let t119927 = t31777 * t2769;
    let t119928 = t786 * t119927;
    let t119930 = t31780 * t122 * t2466;
    let t119931 = t119928 * t119930;
    let t119934 = t822 * t843 * t240;
    (t119914, t119920, t119927, t119928, t119930, t119931, t119934)
}
