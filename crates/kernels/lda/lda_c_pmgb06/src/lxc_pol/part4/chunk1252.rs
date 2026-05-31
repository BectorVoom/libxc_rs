//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1252/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1252<F: Float>(t9619: F, t153: F, t1864: F, t439: F, t4779: F, t4672: F, t6494: F, t4650: F, t6498: F, t2010: F, t4668: F, t1420: F, t6499: F) -> (F, F, F, F, F, F) {
    let t16468 = F::cast_from(2.0_f64) / F::cast_from(135.0_f64) * t9619;
    let t16472 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t439 * t4779 * t153 * t1864;
    let t16475 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t439 * t6494 * t4672;
    let t16478 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t439 * t6498 * t4650;
    let t16481 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t2010 * t6494 * t4668;
    let t16483 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t1420 * t6499;
    (t16468, t16472, t16475, t16478, t16481, t16483)
}
