//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1256/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1256<F: Float>(t1387: F, t6127: F, t493: F, t5486: F, t5493: F, t1447: F, t6509: F, t5499: F, t6513: F, t332: F, t477: F, t6637: F) -> (F, F, F, F, F) {
    let t16518 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t6127 * t1387;
    let t16521 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t493 * t5486 * t5493;
    let t16522 = t1447 * t6509;
    let t16523 = F::cast_from(32.0_f64) / F::cast_from(243.0_f64) * t16522;
    let t16524 = t5499 * t6513;
    let t16525 = F::cast_from(20.0_f64) / F::cast_from(81.0_f64) * t16524;
    let t16527 = t6637 * t477 * t332;
    (t16518, t16521, t16523, t16525, t16527)
}
