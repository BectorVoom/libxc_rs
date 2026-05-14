//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 927/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk927<F: Float>(t1438: F, t7290: F, t332: F, t1901: F, t439: F, t10431: F, t477: F, t7477: F, t15793: F, t15795: F, t1600: F, t7857: F, t1992: F, t493: F, t529: F, t132: F, t435: F, t7812: F) -> (F, F, F, F, F, F, F) {
    let t19489 = t1438 * t7290;
    let t19490 = t19489 * t332;
    let t19493 = t439 * t1901 * t19490 / 27.0;
    let t19497 = 8.0 / 81.0 * t439 * t10431 * t7477 * t477;
    let t19498 = t15793 / 15.0;
    let t19499 = t15795 / 15.0;
    let t19500 = t1600 * t7857;
    let t19504 = t493 * t1992 * t19500 * t529 / 15.0;
    let t19506 = t132 * t435 * t7812;
    (t19490, t19493, t19497, t19498, t19499, t19504, t19506)
}
