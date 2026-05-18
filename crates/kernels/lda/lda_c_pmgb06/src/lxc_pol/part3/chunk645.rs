//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 645/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk645<F: Float>(t109: F, t978: F, t3703: F, t682: F, t1003: F, t1011: F, t957: F, t967: F, t681: F, t683: F, t1024: F, t1035: F, t634: F) -> (F, F, F, F, F, F, F) {
    let t3834 = t109 * t978;
    let t3842 = t3703 * t682;
    let t3851 = t1003 * t1011;
    let t3858 = t957 * t967;
    let t3859 = t3858 * t681;
    let t3862 = t683 * t957;
    let t3867 = F::new(6.0) * t1024 * t634 * t1035;
    (t3834, t3842, t3851, t3858, t3859, t3862, t3867)
}
