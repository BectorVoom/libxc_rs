//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2096/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2096(t28283: f64, t571: f64, t28234: f64, t575: f64, t1455: f64, t7956: f64, t1464: f64, t7939: f64, t2037: f64, t5808: f64, t1921: f64, t7318: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t101656 = 2.0_f64 * t571 * t28283;
    let t101658 = 2.0_f64 * t28234 * t575;
    let t101661 = 2.0_f64 * t1455 * t7956;
    let t101668 = 2.0_f64 * t7939 * t1464;
    let t101670 = 2.0_f64 * t2037 * t5808;
    let t101672 = 2.0_f64 * t7318 * t1921;
    (t101656, t101658, t101661, t101668, t101670, t101672)
}
