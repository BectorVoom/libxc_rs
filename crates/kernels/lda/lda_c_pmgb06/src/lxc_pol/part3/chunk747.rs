//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 747/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk747<F: Float>(t5149: F, t5150: F, t5151: F, t5152: F, t5153: F, t5154: F, t5155: F, t5157: F, t5158: F, t5159: F, t5160: F, t5161: F, t5163: F, t5165: F, t5167: F, t5170: F, t5173: F, t5178: F, t5182: F, t5184: F, t5186: F, t5189: F, t5191: F, t5196: F, t5200: F, t5205: F, t5207: F, t5209: F, t5213: F, t5215: F) -> (F, F) {
    let t5665 = t5149 + t5150 + t5151 - t5152 - t5153 + t5154 - t5155 + t5157 + t5158 + t5159 + t5160 + t5161 + t5163 + t5165 + t5167;
    let t5666 = t5170 + t5173 + t5178 + t5182 + t5184 + t5186 + t5189 + t5191 + t5196 + t5200 + t5205 + t5207 + t5209 - t5213 + t5215;
    (t5665, t5666)
}
