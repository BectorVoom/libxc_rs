//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3366/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3366(t41361: f64, t41363: f64, t51973: f64, t51978: f64, t63325: f64, t63328: f64, t63336: f64, t63338: f64, t63340: f64, t63342: f64, t63346: f64, t63351: f64, t63355: f64) -> f64 {
    let t63426 = -16.0_f64 / 27.0_f64 * t51973 + 56.0_f64 / 81.0_f64 * t51978 + 56.0_f64 / 81.0_f64 * t41361 + 8.0_f64 / 27.0_f64 * t41363 - 40.0_f64 / 27.0_f64 * t63325 + 16.0_f64 / 3.0_f64 * t63328 + 8.0_f64 * t63336 - 8.0_f64 / 9.0_f64 * t63338 + 8.0_f64 / 27.0_f64 * t63340 + 20.0_f64 / 81.0_f64 * t63342 - 10.0_f64 / 27.0_f64 * t63346 - 80.0_f64 / 81.0_f64 * t63351 + 4.0_f64 / 3.0_f64 * t63355;
    t63426
}
