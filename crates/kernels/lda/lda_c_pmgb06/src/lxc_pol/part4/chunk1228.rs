//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1228/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1228<F: Float>(t12112: F, t4836: F, t802: F, t4830: F, t1554: F, t161: F, t2600: F, t132: F, t435: F, t6583: F, t6571: F, t16145: F, t16149: F, t16151: F, t16153: F, t16157: F, t16159: F, t16162: F, t16167: F, t16171: F) -> (F, F, F, F, F, F, F) {
    let t16172 = F::new(4.0) / F::new(135.0) * t12112;
    let t16173 = t802 * t4836;
    let t16174 = F::new(2.0) / F::new(135.0) * t16173;
    let t16176 = F::new(4.0) / F::new(45.0) * t802 * t4830;
    let t16178 = t161 * t1554 * t2600;
    let t16179 = F::new(2.0) / F::new(135.0) * t16178;
    let t16181 = t132 * t435 * t6583;
    let t16182 = F::new(2.0) / F::new(45.0) * t16181;
    let t16184 = t132 * t435 * t6571;
    let t16185 = F::new(4.0) / F::new(45.0) * t16184;
    let t16186 = -t16145 + t16149 - t16151 - t16153 + t16157 - t16159 - t16162 + t16167 - t16171 + t16172 + t16174 + t16176 - t16179 - t16182 - t16185;
    (t16172, t16174, t16176, t16179, t16182, t16185, t16186)
}
