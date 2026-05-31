//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1067/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1067<F: Float>(t132: F, t137: F, t822: F, t9590: F, t1436: F, t1439: F, t2010: F, t332: F, t1423: F, t4767: F, t1558: F, t442: F) -> (F, F, F, F) {
    let t12672 = t132 * t137 * t9590 * t822 / F::cast_from(30.0_f64);
    let t12676 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t2010 * t1436 * t1439 * t332;
    let t12677 = t1423 * t4767;
    let t12678 = F::cast_from(2.0_f64) / F::cast_from(5.0_f64) * t12677;
    let t12682 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t2010 * t442 * t1558 * t332;
    (t12672, t12676, t12678, t12682)
}
