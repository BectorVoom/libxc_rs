//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 981/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk981<F: Float>(t69: F, t8381: F, t8357: F, t8312: F, t1035: F, t1041: F, t1043: F, t217: F, t219: F, t1024: F, t633: F, t3952: F, t654: F) -> (F, F, F, F, F, F, F, F) {
    let t8433 = t69 * t8381;
    let t8439 = t69 * t8357;
    let t8441 = t69 * t8312;
    let t8479 = t1035 * t1035;
    let t8482 = F::cast_from(48.245938496077606_f64) * t1041 * t8479 * t1043;
    let t8485 = F::cast_from(1.0_f64) / t217;
    let t8499 = F::cast_from(1.0_f64) / t219;
    let t8519 = F::cast_from(6.0_f64) * t1024 * t8479 * t633;
    let t8520 = t3952 * t654;
    (t8433, t8439, t8441, t8482, t8485, t8499, t8519, t8520)
}
