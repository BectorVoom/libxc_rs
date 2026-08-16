//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1262/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1262<F: Float>(t12831: F, t9762: F, t9765: F, t1554: F, t161: F, t2554: F, t517: F, t6831: F, t166: F, t529: F, t1586: F, t6230: F) -> (F, F, F, F, F, F) {
    let t16589 = F::cast_from(4.0_f64) / F::cast_from(135.0_f64) * t12831;
    let t16590 = F::cast_from(8.0_f64) / F::cast_from(405.0_f64) * t9762;
    let t16591 = F::cast_from(8.0_f64) / F::cast_from(405.0_f64) * t9765;
    let t16593 = t161 * t1554 * t2554;
    let t16594 = t16593 / F::cast_from(135.0_f64);
    let t16595 = t6831 * t517;
    let t16599 = t161 * t166 * t16595 * t529 / F::cast_from(15.0_f64);
    let t16603 = t161 * t166 * t6230 * t1586 / F::cast_from(30.0_f64);
    (t16589, t16590, t16591, t16594, t16599, t16603)
}
