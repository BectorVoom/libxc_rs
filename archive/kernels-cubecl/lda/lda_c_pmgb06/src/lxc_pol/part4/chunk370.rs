//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 370/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk370<F: Float>(t399: F, t415: F, t398: F, t409: F, t419: F, t421: F, t117: F, t1184: F) -> (F, F, F, F) {
    let t1341 = t399 * t415;
    let t1343 = t409 * t398;
    let t1345 = t1343 * t419 * t421;
    let t1347 = t1184 * t117;
    (t1341, t1343, t1345, t1347)
}
