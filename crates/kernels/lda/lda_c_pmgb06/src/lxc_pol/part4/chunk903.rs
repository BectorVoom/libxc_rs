//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 903/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk903<F: Float>(t493: F, t6545: F, t1969: F, t2002: F, t136: F, t813: F, t1968: F, t439: F, t1592: F, t2648: F, t477: F, t1966: F) -> (F, F, F, F, F, F, F, F) {
    let t6547 = t493 * t6545 / F::new(45.0);
    let t6549 = F::new(2.0) / F::new(15.0) * t2002 * t1969;
    let t6550 = t136 * t813;
    let t6551 = t6550 * t1968;
    let t6553 = F::new(2.0) / F::new(15.0) * t439 * t6551;
    let t6554 = t1592 * t2648;
    let t6555 = t6554 * t477;
    let t6556 = t1966 * t6555;
    (t6547, t6549, t6550, t6551, t6553, t6554, t6555, t6556)
}
