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
    let t15371 = F::new(0.010075555555555556) * t12358 - F::new(0.0008396296296296296) * t12360 - F::new(0.0013993827160493828) * t12362 - F::new(0.007556666666666666) * t12364 - F::new(0.006717037037037037) * t12366 + F::new(0.002239012345679012) * t12368 - F::new(0.04534) * t15347 + F::new(0.011335) * t15351 - F::new(0.003778333333333333) * t15355 - F::new(0.007556666666666666) * t15360 + F::new(0.002518888888888889) * t15365 + F::new(0.0012594444444444445) * t15369;
    (t15365, t15367, t15369, t15371)
}
