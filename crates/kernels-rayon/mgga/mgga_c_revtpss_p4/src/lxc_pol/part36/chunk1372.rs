//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1372/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1372(t101252: f64, t104203: f64, t104208: f64, t108966: f64, t108990: f64, t111639: f64, t111665: f64, t111670: f64, t114260: f64, t114296: f64, t2123: f64, t26792: f64, t28154: f64, t29380: f64, t29388: f64, t29412: f64, t29548: f64, t29562: f64) -> f64 {
    let t116821 = -15.0_f64 * t104208 * t29562 - 15.0_f64 * t104203 * t29562 - 15.0_f64 * t26792 * t114260 + 5.0_f64 / 2.0_f64 * t29388 * t29548 + t114296 * t2123 + 5.0_f64 / 2.0_f64 * t29412 * t29548 + 30.0_f64 * t101252 * t111639 - 10.0_f64 * t108966 * t29380 - 5.0_f64 * t108990 * t29380 - 10.0_f64 * t28154 * t111665 - 10.0_f64 * t28154 * t111670;
    t116821
}
