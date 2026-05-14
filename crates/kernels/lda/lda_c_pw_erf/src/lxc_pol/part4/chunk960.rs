//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 960/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk960<F: Float>(t10463: F, t1325: F, t1328: F, t3783: F, t529: F, t1314: F, t519: F, t3454: F, t518: F, t4048: F, t9: F, t3892: F, t1245: F, t187: F, t22: F, t1479: F, t3762: F, t571: F) -> (F, F, F, F, F, F, F, F) {
    let t10465 = t1325 * t10463 * t1328;
    let t10467 = t3783 * t529;
    let t10469 = t519 * t10467 * t1314;
    let t10474 = t3454 * t518;
    let t10527 = t9 * t4048;
    let t10557 = t9 * t3892;
    let t10567 = t22 / t187 / t1245;
    let t10603 = t571 * t3762 * t1479;
    (t10465, t10467, t10469, t10474, t10527, t10557, t10567, t10603)
}
