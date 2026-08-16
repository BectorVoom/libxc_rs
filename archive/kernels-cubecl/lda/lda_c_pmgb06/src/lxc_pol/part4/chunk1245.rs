//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1245/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1245<F: Float>(t1894: F, t5187: F, t2002: F, t5287: F, t5291: F, t5295: F, t6275: F, t1898: F, t1925: F, t5305: F, t1972: F, t5477: F) -> (F, F, F, F, F, F, F) {
    let t16396 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t5187 * t1894;
    let t16398 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t2002 * t5287;
    let t16400 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t2002 * t5291;
    let t16402 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t6275 * t5295;
    let t16404 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t5187 * t1898;
    let t16406 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t5305 * t1925;
    let t16408 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t1972 * t5477;
    (t16396, t16398, t16400, t16402, t16404, t16406, t16408)
}
