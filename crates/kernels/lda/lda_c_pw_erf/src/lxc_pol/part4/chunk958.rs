//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 958/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk958<F: Float>(t1458: F, t155: F, t1461: F, t519: F, t1335: F, t3762: F, t571: F, t549: F, t593: F, t1401: F, t1485: F, t1620: F, t598: F, t226: F, t4232: F, t1159: F, t603: F) -> (F, F, F, F, F, F, F, F) {
    let t10313 = t155 * t1458;
    let t10315 = t519 * t10313 * t1461;
    let t10361 = t571 * t3762 * t1335;
    let t10392 = t549 * t593;
    let t10397 = t1485 * t1401;
    let t10409 = t598 * t1620;
    let t10412 = 16.0 / 3.0 * t226 * t4232;
    let t10414 = t1159 * t603;
    (t10313, t10315, t10361, t10392, t10397, t10409, t10412, t10414)
}
