//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1047/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1047<F: Float>(t1786: F, t1789: F, t2368: F, t409: F, t328: F, t5915: F, t248: F, t4515: F, t686: F, t2128: F, t642: F, t2136: F) -> (F, F, F, F, F) {
    let t10990 = t409 * t2368 * t1786 * t1789;
    let t10993 = t5915 * t328;
    let t11007 = t248 * t4515 * t686;
    let t11032 = F::cast_from(32.0_f64) * t2128 * t642;
    let t11058 = F::cast_from(32.0_f64) * t2136 * t642;
    (t10990, t10993, t11007, t11032, t11058)
}
