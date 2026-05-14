//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1081/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1081<F: Float>(t16161: F, t2088: F, t1601: F, t161: F, t166: F, t4839: F, t497: F, t843: F, t12112: F, t4836: F, t802: F, t4830: F, t1554: F, t2600: F, t132: F, t435: F, t6583: F) -> (F, F, F, F, F, F, F, F) {
    let t16162 = 2.0 / 135.0 * t16161;
    let t16163 = t2088 * t2088;
    let t16167 = 2.0 / 15.0 * t161 * t166 * t1601 * t16163;
    let t16171 = 4.0 / 45.0 * t161 * t4839 * t843 * t497;
    let t16172 = 4.0 / 135.0 * t12112;
    let t16173 = t802 * t4836;
    let t16174 = 2.0 / 135.0 * t16173;
    let t16176 = 4.0 / 45.0 * t802 * t4830;
    let t16178 = t161 * t1554 * t2600;
    let t16179 = 2.0 / 135.0 * t16178;
    let t16181 = t132 * t435 * t6583;
    (t16162, t16167, t16171, t16172, t16174, t16176, t16179, t16181)
}
