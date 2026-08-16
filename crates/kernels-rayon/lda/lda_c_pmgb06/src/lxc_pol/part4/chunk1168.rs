//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1168/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1168(t1525: f64, t15363: f64, t36: f64, t1074: f64, t6164: f64, t12358: f64, t12360: f64, t12362: f64, t12364: f64, t12366: f64, t12368: f64, t15347: f64, t15351: f64, t15355: f64, t15360: f64) -> (f64, f64, f64, f64) {
    let t15365 = t36 * t1525 * t15363;
    let t15367 = t6164 * t1074;
    let t15369 = t36 * t1525 * t15367;
    let t15371 = 0.010075555555555556_f64 * t12358 - 0.0008396296296296296_f64 * t12360 - 0.0013993827160493828_f64 * t12362 - 0.007556666666666666_f64 * t12364 - 0.006717037037037037_f64 * t12366 + 0.002239012345679012_f64 * t12368 - 0.04534_f64 * t15347 + 0.011335_f64 * t15351 - 0.003778333333333333_f64 * t15355 - 0.007556666666666666_f64 * t15360 + 0.002518888888888889_f64 * t15365 + 0.0012594444444444445_f64 * t15369;
    (t15365, t15367, t15369, t15371)
}
