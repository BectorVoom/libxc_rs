//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1082/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1082<F: Float>(t16181: F, t132: F, t435: F, t6571: F, t16145: F, t16149: F, t16151: F, t16153: F, t16157: F, t16159: F, t16162: F, t16167: F, t16171: F, t16172: F, t16174: F, t16176: F, t16179: F) -> (F, F, F) {
    let t16182 = 2.0 / 45.0 * t16181;
    let t16184 = t132 * t435 * t6571;
    let t16185 = 4.0 / 45.0 * t16184;
    let t16186 = -t16145 + t16149 - t16151 - t16153 + t16157 - t16159 - t16162 + t16167 - t16171 + t16172 + t16174 + t16176 - t16179 - t16182 - t16185;
    (t16182, t16185, t16186)
}
