//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1056/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1056<F: Float>(t12558: F, t1915: F, t1981: F, t2912: F, t764: F, t9525: F, t493: F, t5470: F, t1: F, t1080: F, t2918: F, t1919: F) -> (F, F, F, F, F) {
    let t12561 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t1981 * t1915 * t12558;
    let t12563 = t9525 * t764 * t2912;
    let t12566 = F::cast_from(32.0_f64) / F::cast_from(27.0_f64) * t493 * t5470 * t12563;
    let t12568 = t2918 * t1 * t1080;
    let t12571 = F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1981 * t1919 * t12568;
    (t12561, t12563, t12566, t12568, t12571)
}
