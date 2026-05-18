//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 657/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk657<F: Float>(t1623: F, t405: F, t1554: F, t530: F, t161: F, t1587: F, t489: F, t516: F) -> (F, F, F, F, F, F, F) {
    let t3428 = t405 * t1623;
    let t3450 = t1554 * t530;
    let t3451 = t161 * t3450;
    let t3453 = t489 * t1587;
    let t3454 = t161 * t3453;
    let t3456 = t516 * t516;
    let t3457 = F::new(1.0) / t3456;
    (t3428, t3450, t3451, t3453, t3454, t3456, t3457)
}
