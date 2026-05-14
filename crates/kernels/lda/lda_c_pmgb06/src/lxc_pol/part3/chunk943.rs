//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 943/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk943<F: Float>(t10445: F, t2911: F, t2912: F, t493: F, t851: F, t1894: F, t3177: F, t1420: F, t5287: F, t5268: F, t2948: F, t439: F, t5267: F, t1074: F, t1385: F, t5231: F) -> (F, F, F, F, F, F) {
    let t12724 = 8.0 / 81.0 * t493 * t10445 * t851 * t2911 * t2912;
    let t12726 = t3177 * t1894 / 15.0;
    let t12728 = 2.0 / 15.0 * t1420 * t5287;
    let t12730 = t1420 * t5268 / 15.0;
    let t12733 = t439 * t2948 * t5267 / 15.0;
    let t12737 = t439 * t1385 * t5231 * t1074 / 15.0;
    (t12724, t12726, t12728, t12730, t12733, t12737)
}
