//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1183/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1183(t103: f64, t12360: f64, t12362: f64, t12364: f64, t12366: f64, t12368: f64, t15347: f64, t15351: f64, t15355: f64, t15358: f64, t15363: f64, t15367: f64, t15411: f64, t1619: f64, t3404: f64) -> f64 {
    let t15585 = 0.015996296296296297_f64 * t12360 + 0.026660493827160493_f64 * t12362 + 0.14396666666666666_f64 * t12364 + 0.12797037037037037_f64 * t12366 - 0.04265679012345679_f64 * t12368 + 0.013333333333333334_f64 * t103 * t1619 * t15358 - 0.0044444444444444444_f64 * t103 * t1619 * t15363 - 0.0022222222222222222_f64 * t103 * t1619 * t15367 - 0.002962962962962963_f64 * t103 * t3404 * t15411 + 0.8638_f64 * t15347 - 0.21595_f64 * t15351 + 0.07198333333333333_f64 * t15355;
    t15585
}
