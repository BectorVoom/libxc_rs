//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3821/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3821(t33: f64, t1113: f64, t2: f64, t580: f64, t13565: f64, t14: f64, t20256: f64, t21918: f64, t21923: f64, t27: f64, t3351: f64, t3841: f64, t3842: f64, t47040: f64, t48212: f64, t5557: f64, t6416: f64, t6792: f64, t9342: f64, t9350: f64, zeta_threshold: f64) -> (f64, f64) {
    let t34 = t33 <= zeta_threshold;
    let t73449 = t1113 * t2 * t580;
    let t73470 = piecewise3(t34, 0.0_f64, 40.0_f64 / 81.0_f64 * t47040 * t6792 * t3842 + 64.0_f64 / 27.0_f64 * t13565 * t73449 - 8.0_f64 / 27.0_f64 * t21918 * t3351 + 32.0_f64 / 9.0_f64 * t3841 * t14 * t27 - 16.0_f64 / 9.0_f64 * t5557 * t580 + 16.0_f64 / 3.0_f64 * t5557 * t9342 - 8.0_f64 / 27.0_f64 * t9350 * t6416 * t3842 + 8.0_f64 / 9.0_f64 * t3841 * t20256 * t1113 + 4.0_f64 / 9.0_f64 * t21923 * t3351 - t48212);
    (t73449, t73470)
}
