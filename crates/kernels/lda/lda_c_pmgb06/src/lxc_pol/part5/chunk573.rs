//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 573/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk573<F: Float>(t4197: F, t562: F, t115: F, t1669: F, t1194: F, t113: F, t247: F, t395: F, t2799: F, t2801: F, t1139: F, t199: F, t566: F, t718: F, t26: F, t386: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t4199 = 0.09753333333333333 * t562 * t4197;
    let t4200 = t1669 * t115;
    let t4202 = 0.03145032 * t1194 * t4200;
    let t4205 = 0.001883059277350998 * t113 * t247 * t115;
    let t4206 = 6.0 * t395;
    let t4207 = 18.0 * t2799;
    let t4208 = 12.0 * t2801;
    let t4212 = t1139 * t199;
    let t4214 = t718 * t566;
    let t4230 = t26 * t386;
    (t4199, t4200, t4202, t4205, t4206, t4207, t4208, t4212, t4214, t4230)
}
