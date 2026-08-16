//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1340/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1340(t105814: f64, t109339: f64, t109345: f64, t109348: f64, t109349: f64, t109351: f64, t114826: f64, t114883: f64, t1458: f64, t1914: f64, t1921: f64, t2038: f64, t2045: f64, t25049: f64, t25072: f64, t3: f64, t30161: f64, t30197: f64, t575: f64, t6937: f64, t6951: f64, t7940: f64, t7956: f64) -> f64 {
    let tv4rho3sigma9 = t114826 * t3 * t575 + t114883 * t1458 + 3.0_f64 * t1914 * t30197 + 3.0_f64 * t1921 * t30161 + t2038 * t25072 + t2045 * t25049 + 3.0_f64 * t6937 * t7956 + 3.0_f64 * t6951 * t7940 + 3.0_f64 * t105814 + 6.0_f64 * t109339 + 3.0_f64 * t109345 + 3.0_f64 * t109348 + 6.0_f64 * t109349 + 3.0_f64 * t109351;
    tv4rho3sigma9
}
