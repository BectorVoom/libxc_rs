//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1744/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1744(t30: f64, t525: f64, t9603: f64, t2257: f64, t3833: f64, t3834: f64, t39456: f64, t46311: f64, t46317: f64, t513: f64, t9335: f64, t9339: f64, t9344: f64, zeta_threshold: f64) -> f64 {
    let t31 = t30 <= zeta_threshold;
    let t47025 = 1.0_f64 / t525 / t9603;
    let t47038 = piecewise3(t31, 0.0_f64, 40.0_f64 / 81.0_f64 * t47025 * t46311 - 16.0_f64 / 9.0_f64 * t9335 * t3834 * t2257 + 4.0_f64 / 3.0_f64 * t3833 * t46317 + 16.0_f64 / 9.0_f64 * t9339 * t9344 + 4.0_f64 / 3.0_f64 * t513 * t39456);
    t47038
}
