//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 778/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk778(t3976: f64, t5155: f64, t593: f64, t3974: f64, t1333: f64, t4574: f64, t352: f64, t1484: f64, t219: f64, t1351: f64, t2066: f64, t514: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5157 = t3976 * t5155 * t593;
    let t5159 = 16.0_f64 / 45.0_f64 * t3974 * t5157;
    let t5160 = t4574 * t1333;
    let t5161 = t5155 * t352;
    let t5162 = t5160 * t5161;
    let t5164 = 32.0_f64 / 45.0_f64 * t3974 * t5162;
    let t5165 = t1484 * t219;
    let t5166 = t5165 * t1351;
    let t5167 = t5166 * t5161;
    let t5169 = 16.0_f64 / 27.0_f64 * t3974 * t5167;
    let t5170 = t514 * t2066;
    (t5157, t5159, t5160, t5162, t5164, t5165, t5166, t5167, t5169, t5170)
}
