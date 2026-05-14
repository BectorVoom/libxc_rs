//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 833/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk833<F: Float>(t477: F, t6554: F, t1966: F, t439: F, t1601: F, t497: F, t764: F, t851: F) -> (F, F, F, F, F) {
    let t6555 = t6554 * t477;
    let t6556 = t1966 * t6555;
    let t6558 = t439 * t6556 / 15.0;
    let t6559 = t1601 * t497;
    let t6560 = t764 * t851;
    (t6555, t6556, t6558, t6559, t6560)
}
