//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 750/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk750<F: Float>(t1972: F, t2462: F, t6112: F, t851: F, t1992: F, t493: F, t2002: F, t2477: F, t6123: F, t805: F, t439: F, t6127: F, t806: F, t6554: F, t822: F, t1966: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7683 = 2.0 / 15.0 * t1972 * t2462;
    let t7684 = t6112 * t851;
    let t7685 = t1992 * t7684;
    let t7687 = t493 * t7685 / 5.0;
    let t7689 = 2.0 / 15.0 * t2002 * t2477;
    let t7690 = t6123 * t805;
    let t7692 = t439 * t7690 / 15.0;
    let t7694 = t6127 * t806 / 15.0;
    let t7695 = t6554 * t822;
    let t7696 = t1966 * t7695;
    (t7683, t7684, t7685, t7687, t7689, t7690, t7692, t7694, t7695, t7696)
}
