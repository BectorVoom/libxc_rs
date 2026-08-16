//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1071/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1071<F: Float>(t1080: F, t1464: F, t2088: F, t2991: F, t493: F, t10445: F, t2911: F, t2912: F, t851: F, t1894: F, t3177: F, t1420: F, t5287: F) -> (F, F, F, F) {
    let t12719 = t493 * t2991 * t2088 * t1464 * t1080 / F::cast_from(9.0_f64);
    let t12724 = F::cast_from(8.0_f64) / F::cast_from(81.0_f64) * t493 * t10445 * t851 * t2911 * t2912;
    let t12726 = t3177 * t1894 / F::cast_from(15.0_f64);
    let t12728 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t1420 * t5287;
    (t12719, t12724, t12726, t12728)
}
