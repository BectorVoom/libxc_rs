//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1002/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1002<F: Float>(t1347: F, t1795: F, t118: F, t5575: F, t2174: F, t415: F, t14239: F, t5522: F, t2249: F, t384: F, t387: F, t5887: F, t707: F, t5891: F, t5895: F, t1770: F, t419: F, t4238: F, t794: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t14541 = t1795 * t1347;
    let t14543 = t5575 * t118;
    let t14545 = t2174 * t415;
    let t14547 = t14239 * t118;
    let t14549 = t5522 * t415;
    let t14561 = t387 * t384 * t2249;
    let t14567 = t707 * t5887;
    let t14569 = t707 * t5891;
    let t14571 = t707 * t5895;
    let t14575 = t4238 * t794 * t419 * t1770;
    (t14541, t14543, t14545, t14547, t14549, t14561, t14567, t14569, t14571, t14575)
}
