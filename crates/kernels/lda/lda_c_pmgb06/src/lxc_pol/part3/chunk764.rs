//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 764/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk764<F: Float>(t446: F, t5187: F, t1420: F, t1969: F, t5157: F, t5158: F, t5159: F, t5160: F, t5161: F, t5163: F, t5165: F, t5167: F, t5170: F, t5173: F, t5178: F, t5182: F, t5184: F, t5186: F) -> (F, F, F) {
    let t5189 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t5187 * t446;
    let t5191 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t1420 * t1969;
    let t5192 = t5157 + t5158 + t5159 + t5160 + t5161 + t5163 + t5165 + t5167 + t5170 + t5173 + t5178 + t5182 + t5184 + t5186 + t5189 + t5191;
    (t5189, t5191, t5192)
}
