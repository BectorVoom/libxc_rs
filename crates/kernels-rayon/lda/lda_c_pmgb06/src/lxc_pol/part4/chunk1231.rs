//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1231/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1231(t16213: f64, t1898: f64, t5220: f64, t1902: f64, t5211: f64, t6478: f64, t16187: f64, t16189: f64, t16190: f64, t16192: f64, t16195: f64, t16199: f64, t16201: f64, t16204: f64, t16207: f64, t16210: f64, t16212: f64) -> (f64, f64, f64, f64, f64) {
    let t16214 = 8.0_f64 / 135.0_f64 * t16213;
    let t16215 = t5220 * t1898;
    let t16216 = 16.0_f64 / 135.0_f64 * t16215;
    let t16217 = t5220 * t1902;
    let t16218 = 8.0_f64 / 81.0_f64 * t16217;
    let t16219 = t5211 * t6478;
    let t16220 = 20.0_f64 / 81.0_f64 * t16219;
    let t16221 = t16187 - t16189 - t16190 - t16192 - t16195 - t16199 - t16201 - t16204 - t16207 - t16210 - t16212 - t16214 - t16216 + t16218 - t16220;
    (t16214, t16216, t16218, t16220, t16221)
}
