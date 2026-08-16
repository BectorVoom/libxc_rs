//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1168/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1168<F: Float>(t1525: F, t15363: F, t36: F, t1074: F, t6164: F, t12358: F, t12360: F, t12362: F, t12364: F, t12366: F, t12368: F, t15347: F, t15351: F, t15355: F, t15360: F) -> (F, F, F, F) {
    let t15365 = t36 * t1525 * t15363;
    let t15367 = t6164 * t1074;
    let t15369 = t36 * t1525 * t15367;
    let t15371 = F::cast_from(0.010075555555555556_f64) * t12358 - F::cast_from(0.0008396296296296296_f64) * t12360 - F::cast_from(0.0013993827160493828_f64) * t12362 - F::cast_from(0.007556666666666666_f64) * t12364 - F::cast_from(0.006717037037037037_f64) * t12366 + F::cast_from(0.002239012345679012_f64) * t12368 - F::cast_from(0.04534_f64) * t15347 + F::cast_from(0.011335_f64) * t15351 - F::cast_from(0.003778333333333333_f64) * t15355 - F::cast_from(0.007556666666666666_f64) * t15360 + F::cast_from(0.002518888888888889_f64) * t15365 + F::cast_from(0.0012594444444444445_f64) * t15369;
    (t15365, t15367, t15369, t15371)
}
