//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 992/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk992<F: Float>(t1444: F, t7509: F, t2979: F, t493: F, t7508: F, t1380: F, t6827: F, t851: F, t20569: F, t20572: F, t20575: F, t20577: F, t20579: F, t20581: F, t20584: F, t20587: F) -> (F, F, F, F) {
    let t20589 = t1444 * t7509 / 15.0;
    let t20592 = t493 * t2979 * t7508 / 15.0;
    let t20596 = t493 * t1380 * t6827 * t851 / 15.0;
    let t20597 = t20569 + t20572 - t20575 + t20577 + t20579 - t20581 - t20584 + t20587 - t20589 - t20592 - t20596;
    (t20589, t20592, t20596, t20597)
}
