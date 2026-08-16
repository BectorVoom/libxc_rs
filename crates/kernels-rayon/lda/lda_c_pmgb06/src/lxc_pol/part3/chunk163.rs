//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 163/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk163(t5: f64, t418: f64, t419: f64, t421: f64, t117: f64, t123: f64, t191: f64, t315: f64, t332: f64, t44: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t6 = t5 <= zeta_threshold;
    let t423 = 0.001975389032890948_f64 * t418 * t419 * t421;
    let t427 = 0.008980675507690957_f64 * t123 * t315 * t191 * t117;
    let t430 = piecewise3(t6, 0.0_f64, 2.0_f64 * t5 * t332);
    let t431 = t430 * t44;
    (t423, t427, t431)
}
