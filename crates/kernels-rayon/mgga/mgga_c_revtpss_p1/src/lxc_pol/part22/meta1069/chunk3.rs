//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3825/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3825(t30: f64, t13687: f64, t14: f64, t18280: f64, t21944: f64, t21949: f64, t2257: f64, t27: f64, t3834: f64, t3874: f64, t46310: f64, t48394: f64, t5574: f64, t580: f64, t5824: f64, t605: f64, t6785: f64, t73423: f64, t9342: f64, t9605: f64, zeta_threshold: f64) -> f64 {
    let t31 = t30 <= zeta_threshold;
    let t73552 = piecewise3(t31, 0.0_f64, -56.0_f64 / 81.0_f64 * t46310 * t6785 * t3834 + 64.0_f64 / 27.0_f64 * t13687 * t73423 + 8.0_f64 / 27.0_f64 * t21944 * t2257 - 16.0_f64 / 9.0_f64 * t3874 * t14 * t27 - 8.0_f64 / 9.0_f64 * t5574 * t580 + 8.0_f64 / 3.0_f64 * t5574 * t9342 + 8.0_f64 / 27.0_f64 * t9605 * t5824 * t3834 - 4.0_f64 / 9.0_f64 * t3874 * t18280 * t605 - 2.0_f64 / 9.0_f64 * t21949 * t2257 + t48394);
    t73552
}
