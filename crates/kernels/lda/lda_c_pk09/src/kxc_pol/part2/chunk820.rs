//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 820/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk820<F: Float>(t1290: F, t9836: F, t1402: F, t9602: F, t1287: F, t741: F, t7766: F, t93: F, t10: F, t1214: F, t407: F, t130: F, t9739: F, t1345: F, t1434: F, t2649: F) -> (F, F, F, F, F, F, F) {
    let t9837 = t1290 * t9836;
    let t9839 = t1402 * t9602;
    let t9840 = t9839 * t1287;
    let t9842 = t741 * t7766;
    let t9843 = t93 * t9842;
    let t9846 = t1214 * t10;
    let t9847 = t407 * t9846;
    let t9850 = t130 * t9739;
    let t9851 = t93 * t9850;
    let t9854 = t1345 * t9836;
    let t9856 = t1434 * t2649;
    (t9837, t9840, t9843, t9847, t9851, t9854, t9856)
}
