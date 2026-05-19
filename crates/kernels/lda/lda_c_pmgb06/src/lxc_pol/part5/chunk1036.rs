//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1036/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1036<F: Float>(t19395: F, t497: F, t36: F, t506: F, t350: F, t7595: F, t7599: F, t13370: F, t13373: F, t13399: F, t13407: F, t17131: F, t17133: F, t17138: F, t17140: F, t17164: F, t17166: F, t17177: F, t19379: F, t19383: F, t19387: F, t19391: F, t9502: F, t9552: F) -> (F, F, F, F, F) {
    let t19396 = t497 * t19395;
    let t19398 = t36 * t506 * t19396;
    let t19400 = t350 * t7595;
    let t19402 = t350 * t7599;
    let t19414 = F::cast_from(0.005597530864197531_f64) * t19379 - F::cast_from(0.012594444444444445_f64) * t19383 + F::new(0.04534) * t19387 + F::new(0.04534) * t19391 - F::cast_from(0.005037777777777778_f64) * t13370 + t13373 + F::cast_from(0.0018891666666666666_f64) * t19398 - F::cast_from(0.0006996913580246914_f64) * t19400 + F::cast_from(0.002518888888888889_f64) * t19402 - t9502 - F::cast_from(0.015113333333333333_f64) * t17131 + F::cast_from(0.007556666666666666_f64) * t17133 + F::cast_from(0.003778333333333333_f64) * t17138 - F::cast_from(0.0012594444444444445_f64) * t17140 - F::cast_from(0.002099074074074074_f64) * t17164 + F::cast_from(0.005037777777777778_f64) * t17166 - F::new(0.011335) * t17177 - F::cast_from(0.005037777777777778_f64) * t13399 - F::cast_from(0.005877407407407408_f64) * t13407 - F::cast_from(0.0019591358024691357_f64) * t9552;
    (t19396, t19398, t19400, t19402, t19414)
}
