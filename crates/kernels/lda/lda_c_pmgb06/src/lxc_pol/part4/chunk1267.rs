//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1267/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1267<F: Float>(t15382: F, t439: F, t5260: F, t12154: F, t15387: F, t5168: F, t6478: F, t2010: F, t5253: F, t6155: F, t1420: F, t6416: F) -> (F, F, F, F, F) {
    let t16649 = F::cast_from(8.0_f64) / F::cast_from(81.0_f64) * t439 * t5260 * t15382;
    let t16652 = F::cast_from(88.0_f64) / F::cast_from(243.0_f64) * t439 * t12154 * t15387;
    let t16654 = F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t5168 * t6478;
    let t16657 = F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t2010 * t5253 * t6155;
    let t16659 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t1420 * t6416;
    (t16649, t16652, t16654, t16657, t16659)
}
