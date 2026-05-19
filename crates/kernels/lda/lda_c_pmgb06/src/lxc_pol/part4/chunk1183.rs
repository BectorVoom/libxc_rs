//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1183/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1183<F: Float>(t103: F, t12360: F, t12362: F, t12364: F, t12366: F, t12368: F, t15347: F, t15351: F, t15355: F, t15358: F, t15363: F, t15367: F, t15411: F, t1619: F, t3404: F) -> F {
    let t15585 = F::cast_from(0.015996296296296297_f64) * t12360 + F::cast_from(0.026660493827160493_f64) * t12362 + F::cast_from(0.14396666666666666_f64) * t12364 + F::cast_from(0.12797037037037037_f64) * t12366 - F::cast_from(0.04265679012345679_f64) * t12368 + F::cast_from(0.013333333333333334_f64) * t103 * t1619 * t15358 - F::cast_from(0.0044444444444444444_f64) * t103 * t1619 * t15363 - F::cast_from(0.0022222222222222222_f64) * t103 * t1619 * t15367 - F::cast_from(0.002962962962962963_f64) * t103 * t3404 * t15411 + F::new(0.8638) * t15347 - F::new(0.21595) * t15351 + F::cast_from(0.07198333333333333_f64) * t15355;
    t15585
}
