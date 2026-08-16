//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 690/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk690<F: Float>(t5: F, t387: F, t73: F, t2249: F, t3537: F, t760: F, t1: F, t1212: F, t332: F, t395: F, t1069: F, t1074: F, t2192: F, t2195: F, t247: F, t330: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t6 = t5 <= zeta_threshold;
    let t4359 = t387 * t73;
    let t4360 = t4359 * t2249;
    let t4363 = t3537 * t760;
    let t4366 = t1212 * t1;
    let t4367 = t332 * t395;
    let t4377 = piecewise3::<F>(t6, F::cast_from(0.0_f64), F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t4363 * t1069 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t4366 * t4367 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t2192 * t1074 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t330 * t395 - F::cast_from(4.0_f64) * t2195 * t247);
    (t4359, t4360, t4363, t4366, t4367, t4377)
}
