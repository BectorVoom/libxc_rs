//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 618/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk618<F: Float>(t119: F, t155: F, t1664: F, t3210: F, t411: F, t473: F, t1691: F, t156: F, t1678: F, t426: F, t427: F, t474: F, t1682: F, t259: F, t47: F, t261: F, t52: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3212 = t119 * t155 * t1664;
    let t3213 = t3210 * t3212;
    let t3216 = t119 * t473 * t411;
    let t3217 = t1691 * t3216;
    let t3219 = t156 * t1678;
    let t3220 = t426 * t3219;
    let t3227 = t474 * t427;
    let t3228 = t426 * t3227;
    let t3230 = t156 * t1682;
    let t3231 = t426 * t3230;
    let t3234 = 1.0 / t47 / t259;
    let t3243 = 1.0 / t52 / t261;
    (t3212, t3213, t3216, t3217, t3219, t3220, t3227, t3228, t3230, t3231, t3234, t3243)
}
