//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1064/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1064<F: Float>(t2257: F, t26: F, t329: F, t1322: F, t5882: F, t301: F, t413: F, t5567: F, t642: F, t794: F, t113: F, t1329: F, t1808: F) -> (F, F, F, F, F, F, F) {
    let t11639 = t26 * t2257;
    let t11640 = t329 * t11639;
    let t11645 = t5882 * t1322;
    let t11674 = t5567 * t413 * t301;
    let t11676 = t642 * t794;
    let t11678 = t11676 * t113 * t301;
    let t11698 = t1329 * t1808;
    (t11639, t11640, t11645, t11674, t11676, t11678, t11698)
}
