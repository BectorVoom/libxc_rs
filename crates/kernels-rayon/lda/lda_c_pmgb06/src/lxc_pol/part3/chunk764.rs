//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 764/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk764(t446: f64, t5187: f64, t1420: f64, t1969: f64, t5157: f64, t5158: f64, t5159: f64, t5160: f64, t5161: f64, t5163: f64, t5165: f64, t5167: f64, t5170: f64, t5173: f64, t5178: f64, t5182: f64, t5184: f64, t5186: f64) -> (f64, f64, f64) {
    let t5189 = 2.0_f64 / 45.0_f64 * t5187 * t446;
    let t5191 = 2.0_f64 / 15.0_f64 * t1420 * t1969;
    let t5192 = t5157 + t5158 + t5159 + t5160 + t5161 + t5163 + t5165 + t5167 + t5170 + t5173 + t5178 + t5182 + t5184 + t5186 + t5189 + t5191;
    (t5189, t5191, t5192)
}
