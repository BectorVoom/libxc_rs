//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 566/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk566<F: Float>(t1122: F, t3969: F, t1139: F, t301: F, t413: F, t1183: F, t718: F, t247: F, t398: F, t113: F, t1135: F, t100: F, t641: F) -> (F, F, F, F, F, F, F) {
    let t3970 = t3969 * t1122;
    let t3987 = t1139 * t413 * t301;
    let t3991 = 0.0008717022455366076 * t718 * t1183 * t301;
    let t3993 = t247 * t398;
    let t3995 = t3993 * t113 * t301;
    let t3999 = 0.004067943812504169 * t1135 * t413 * t301;
    let t4001 = 1.0 / t100 / t641;
    (t3970, t3987, t3991, t3993, t3995, t3999, t4001)
}
