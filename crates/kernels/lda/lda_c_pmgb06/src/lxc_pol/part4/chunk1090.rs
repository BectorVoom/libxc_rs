//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1090/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1090<F: Float>(t12912: F, t500: F, t1451: F, t5194: F, t1455: F, t1467: F, t1944: F, t642: F, t1952: F, t12514: F, t1461: F, t5065: F) -> (F, F, F, F, F, F, F) {
    let t12913 = t12912 * t500;
    let t12915 = t5194 * t1451;
    let t12917 = t5194 * t1455;
    let t12919 = t5194 * t1467;
    let t12939 = F::new(64.0) * t1944 * t642;
    let t12960 = F::new(64.0) * t1952 * t642;
    let t12981 = t5065 * t12514 * t1461;
    (t12913, t12915, t12917, t12919, t12939, t12960, t12981)
}
