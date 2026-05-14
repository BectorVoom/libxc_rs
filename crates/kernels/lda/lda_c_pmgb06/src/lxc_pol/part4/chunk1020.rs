//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1020/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1020<F: Float>(t2695: F, t374: F, t4232: F, t4359: F, t7077: F, t1322: F, t7088: F, t297: F, t301: F, t413: F, t6716: F, t1183: F, t2414: F, t247: F, t8194: F) -> (F, F, F, F, F, F) {
    let t15086 = t4232 * t2695 * t374;
    let t15089 = t4359 * t7077;
    let t15096 = t7088 * t1322;
    let t15102 = t297 * t6716 * t413 * t301;
    let t15106 = t297 * t2414 * t1183 * t301;
    let t15116 = 24.0 * t247 - t8194;
    (t15086, t15089, t15096, t15102, t15106, t15116)
}
