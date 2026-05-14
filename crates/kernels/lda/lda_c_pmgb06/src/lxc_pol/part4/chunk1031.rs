//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1031/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1031<F: Float>(t1420: F, t6297: F, t2492: F, t3216: F, t439: F, t6300: F, t1454: F, t493: F, t6130: F, t1461: F, t2553: F, t1466: F, t1455: F, t6134: F, t1467: F, t1972: F, t4585: F) -> (F, F, F, F, F, F, F, F) {
    let t15290 = 4.0 / 45.0 * t1420 * t6297;
    let t15293 = 2.0 / 45.0 * t439 * t3216 * t2492;
    let t15295 = 4.0 / 45.0 * t1420 * t6300;
    let t15298 = t493 * t6130 * t1454 / 45.0;
    let t15299 = t1461 * t2553;
    let t15302 = t493 * t15299 * t1466 / 27.0;
    let t15304 = t6134 * t1455 / 45.0;
    let t15306 = t6134 * t1467 / 27.0;
    let t15308 = 2.0 / 45.0 * t1972 * t4585;
    (t15290, t15293, t15295, t15298, t15302, t15304, t15306, t15308)
}
