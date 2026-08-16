//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3820/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3820(t30: f64, t47060: f64, t2: f64, t580: f64, t605: f64, t13550: f64, t14: f64, t18280: f64, t21906: f64, t21911: f64, t2257: f64, t27: f64, t3833: f64, t3834: f64, t47025: f64, t48185: f64, t5549: f64, t5824: f64, t6785: f64, t9335: f64, t9342: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t73418 = 0.11696447245269292414e1_f64 * t47060;
    let t73423 = t605 * t2 * t580;
    let t73444 = piecewise3(t31, 0.0_f64, 40.0_f64 / 81.0_f64 * t47025 * t6785 * t3834 - 64.0_f64 / 27.0_f64 * t13550 * t73423 - 8.0_f64 / 27.0_f64 * t21906 * t2257 + 32.0_f64 / 9.0_f64 * t3833 * t14 * t27 + 16.0_f64 / 9.0_f64 * t5549 * t580 - 16.0_f64 / 3.0_f64 * t5549 * t9342 - 8.0_f64 / 27.0_f64 * t9335 * t5824 * t3834 + 8.0_f64 / 9.0_f64 * t3833 * t18280 * t605 + 4.0_f64 / 9.0_f64 * t21911 * t2257 + t48185);
    (t73418, t73423, t73444)
}
