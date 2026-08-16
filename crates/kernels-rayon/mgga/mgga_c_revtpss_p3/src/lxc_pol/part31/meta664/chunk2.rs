//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2256/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2256(t1913: f64, t7956: f64, t101563: f64, t105814: f64, t109278: f64, t109289: f64, t109334: f64, t1458: f64, t1464: f64, t1914: f64, t1921: f64, t2038: f64, t2045: f64, t22533: f64, t22571: f64, t28235: f64, t28283: f64, t3: f64, t30161: f64, t575: f64, t5790: f64, t5808: f64, t6951: f64, t7319: f64, t7940: f64) -> f64 {
    let t109339 = t1913 * t7956;
    let t109344 = t30161 * t1464 + t22533 * t2045 + t7319 * t6951 + 2.0_f64 * t28235 * t1921 + 2.0_f64 * t5790 * t7956 + t105814 + 2.0_f64 * t1914 * t28283 + t101563 + t1458 * (t109289 + t109334) + t3 * t109278 * t575 + 2.0_f64 * t109339 + 2.0_f64 * t7940 * t5808 + t2038 * t22571;
    t109344
}
