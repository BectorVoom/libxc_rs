//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3375/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3375(t41330: f64, t41332: f64, t52047: f64, t52049: f64, t52051: f64, t63399: f64, t63447: f64, t63451: f64, t63453: f64, t63457: f64, t63459: f64, t63462: f64, t63464: f64) -> f64 {
    let t63466 = 8.0_f64 / 27.0_f64 * t52047 + 4.0_f64 / 27.0_f64 * t52049 + 20.0_f64 / 81.0_f64 * t52051 - 8.0_f64 * t63399 - 4.0_f64 / 27.0_f64 * t41330 - 8.0_f64 / 81.0_f64 * t41332 + 2.0_f64 / 9.0_f64 * t63447 - t63451 / 3.0_f64 - 8.0_f64 / 81.0_f64 * t63453 - 4.0_f64 / 9.0_f64 * t63457 + 8.0_f64 / 27.0_f64 * t63459 + 4.0_f64 / 3.0_f64 * t63462 - 4.0_f64 / 27.0_f64 * t63464;
    t63466
}
