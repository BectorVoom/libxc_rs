//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1078/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1078<F: Float>(t1431: F, t6127: F, t1441: F, t1430: F, t439: F, t6123: F, t1435: F, t2582: F, t1440: F, t12041: F, t1995: F, t5305: F, t1972: F, t5180: F, t136: F, t1872: F, t1968: F) -> (F, F, F, F, F, F, F, F) {
    let t16112 = t6127 * t1431 / 45.0;
    let t16114 = t6127 * t1441 / 27.0;
    let t16117 = t439 * t6123 * t1430 / 45.0;
    let t16118 = t1435 * t2582;
    let t16121 = t439 * t16118 * t1440 / 27.0;
    let t16122 = 16.0 / 1215.0 * t12041;
    let t16124 = 4.0 / 15.0 * t5305 * t1995;
    let t16126 = 4.0 / 15.0 * t1972 * t5180;
    let t16130 = 4.0 / 15.0 * t439 * t136 * t1872 * t1968;
    (t16112, t16114, t16117, t16121, t16122, t16124, t16126, t16130)
}
