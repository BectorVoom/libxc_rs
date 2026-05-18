//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1255/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1255<F: Float>(t493: F, t5463: F, t6503: F, t2501: F, t3213: F, t2979: F, t6782: F, t1444: F, t6783: F, t464: F, t6123: F, t1386: F, t439: F) -> (F, F, F, F, F) {
    let t16505 = F::new(4.0) / F::new(9.0) * t493 * t5463 * t6503;
    let t16506 = t3213 * t2501;
    let t16507 = F::new(4.0) / F::new(405.0) * t16506;
    let t16510 = F::new(2.0) / F::new(45.0) * t493 * t2979 * t6782;
    let t16512 = F::new(2.0) / F::new(45.0) * t1444 * t6783;
    let t16513 = t6123 * t464;
    let t16516 = F::new(2.0) / F::new(45.0) * t439 * t16513 * t1386;
    (t16505, t16507, t16510, t16512, t16516)
}
