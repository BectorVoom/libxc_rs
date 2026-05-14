//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1024/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1024<F: Float>(t15189: F, t1069: F, t1385: F, t1531: F, t2648: F, t439: F, t1908: F, t5220: F, t2002: F, t5345: F, t1080: F, t6764: F, t1915: F, t493: F, t10139: F, t1602: F, t2541: F) -> (F, F, F, F, F, F, F) {
    let t15190 = 4.0 / 135.0 * t15189;
    let t15195 = 2.0 / 45.0 * t439 * t1385 * t2648 * t1531 * t1069;
    let t15196 = t5220 * t1908;
    let t15197 = 8.0 / 135.0 * t15196;
    let t15199 = 4.0 / 45.0 * t2002 * t5345;
    let t15200 = t6764 * t1080;
    let t15203 = 2.0 / 15.0 * t493 * t1915 * t15200;
    let t15207 = 2.0 / 27.0 * t493 * t10139 * t2541 * t1602;
    (t15190, t15195, t15197, t15199, t15200, t15203, t15207)
}
