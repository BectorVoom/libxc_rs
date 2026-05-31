//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 353/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk353<F: Float>(t1234: F, t38: F, t64: F, t1227: F, t56: F, t409: F, t54: F, t55: F, t110: F, t361: F, t360: F, t370: F) -> (F, F, F, F, F, F) {
    let t1252 = F::cast_from(5.84605_f64) * t38 * t64 * t1234;
    let t1255 = F::cast_from(2.923025_f64) * t38 * t56 * t1227;
    let t1259 = t54 * t55 * t409 * t56 / F::cast_from(9.0_f64);
    let t1260 = t110 * t361;
    let t1261 = t360 * t1260;
    let t1263 = t370 * t1234;
    (t1252, t1255, t1259, t1260, t1261, t1263)
}
