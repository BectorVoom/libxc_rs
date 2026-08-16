//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1036/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1036(t19395: f64, t497: f64, t36: f64, t506: f64, t350: f64, t7595: f64, t7599: f64, t13370: f64, t13373: f64, t13399: f64, t13407: f64, t17131: f64, t17133: f64, t17138: f64, t17140: f64, t17164: f64, t17166: f64, t17177: f64, t19379: f64, t19383: f64, t19387: f64, t19391: f64, t9502: f64, t9552: f64) -> (f64, f64, f64, f64, f64) {
    let t19396 = t497 * t19395;
    let t19398 = t36 * t506 * t19396;
    let t19400 = t350 * t7595;
    let t19402 = t350 * t7599;
    let t19414 = 0.005597530864197531_f64 * t19379 - 0.012594444444444445_f64 * t19383 + 0.04534_f64 * t19387 + 0.04534_f64 * t19391 - 0.005037777777777778_f64 * t13370 + t13373 + 0.0018891666666666666_f64 * t19398 - 0.0006996913580246914_f64 * t19400 + 0.002518888888888889_f64 * t19402 - t9502 - 0.015113333333333333_f64 * t17131 + 0.007556666666666666_f64 * t17133 + 0.003778333333333333_f64 * t17138 - 0.0012594444444444445_f64 * t17140 - 0.002099074074074074_f64 * t17164 + 0.005037777777777778_f64 * t17166 - 0.011335_f64 * t17177 - 0.005037777777777778_f64 * t13399 - 0.005877407407407408_f64 * t13407 - 0.0019591358024691357_f64 * t9552;
    (t19396, t19398, t19400, t19402, t19414)
}
