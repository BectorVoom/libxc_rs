//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2270/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2270(t30993: f64, t571: f64, t104094: f64, t111419: f64, t113015: f64, t113019: f64, t113022: f64, t1456: f64, t1464: f64, t1921: f64, t29469: f64, t3: f64, t30975: f64, t575: f64, t5790: f64, t5808: f64, t6937: f64, t6951: f64, t7691: f64, t7700: f64, t8241: f64, t8249: f64) -> f64 {
    let t113025 = t571 * t30993;
    let t113026 = t113015 * t3 * t575 + t1456 * t30993 + t1464 * t30975 + 2.0_f64 * t1921 * t29469 + 2.0_f64 * t5790 * t8249 + 2.0_f64 * t5808 * t8241 + t6937 * t7700 + t6951 * t7691 + t104094 + t111419 + t113019 + 2.0_f64 * t113022 + t113025;
    t113026
}
