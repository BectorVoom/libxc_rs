//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 663/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk663<F: Float>(t109: F, t1234: F, t55: F, t3594: F, t27: F, t348: F, t64: F, t1243: F, t3582: F, t19: F, t369: F) -> (F, F, F, F, F) {
    let t3596 = t55 * t109 * t1234;
    let t3597 = t3594 * t3596;
    let t3600 = t348 * t64 * t27;
    let t3601 = t3600 * t3596;
    let t3603 = t1243 * t3582;
    let t3615 = F::cast_from(1.0_f64) / t369 / t19;
    (t3597, t3600, t3601, t3603, t3615)
}
