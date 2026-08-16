//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1046/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1046<F: Float>(t1444: F, t7663: F, t441: F, t7501: F, t439: F, t445: F, t224: F, t7464: F, t446: F, t15807: F, t1447: F, t7509: F) -> (F, F, F, F, F) {
    let t19514 = t1444 * t7663 / F::cast_from(15.0_f64);
    let t19515 = t441 * t7501;
    let t19518 = t439 * t19515 * t445 / F::cast_from(45.0_f64);
    let t19519 = t7464 * t224;
    let t19521 = t19519 * t446 / F::cast_from(45.0_f64);
    let t19522 = t15807 / F::cast_from(15.0_f64);
    let t19523 = t1447 * t7509;
    (t19514, t19518, t19521, t19522, t19523)
}
