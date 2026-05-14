//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 929/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk929<F: Float>(t19515: F, t439: F, t445: F, t224: F, t7464: F, t446: F, t15807: F, t1447: F, t7509: F, t7513: F, t15829: F, t15831: F, t15835: F, t15838: F, t11898: F, t19514: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t19518 = t439 * t19515 * t445 / 45.0;
    let t19519 = t7464 * t224;
    let t19521 = t19519 * t446 / 45.0;
    let t19522 = t15807 / 15.0;
    let t19523 = t1447 * t7509;
    let t19524 = 2.0 / 45.0 * t19523;
    let t19525 = t1447 * t7513;
    let t19526 = 4.0 / 45.0 * t19525;
    let t19527 = 8.0 / 45.0 * t15829;
    let t19528 = 4.0 / 45.0 * t15831;
    let t19529 = 4.0 / 45.0 * t15835;
    let t19530 = 8.0 / 45.0 * t15838;
    let t19531 = t19514 + t19518 + t19521 + t11898 - t19522 - t19524 - t19526 - t19527 + t19528 + t19529 - t19530;
    (t19518, t19521, t19522, t19524, t19526, t19527, t19528, t19529, t19530, t19531)
}
