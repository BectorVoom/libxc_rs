//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 554/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk554<F: Float>(t1527: F, t350: F, t1533: F, t1537: F, t139: F, t1435: F, t127: F, t1437: F) -> (F, F, F, F, F) {
    let t3084 = t350 * t1527;
    let t3086 = t350 * t1533;
    let t3088 = t350 * t1537;
    let t3090 = t139 * t1435;
    let t3092 = 1.0 / t1437 / t127;
    (t3084, t3086, t3088, t3090, t3092)
}
