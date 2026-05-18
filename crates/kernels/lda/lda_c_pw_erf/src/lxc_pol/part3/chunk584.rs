//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 584/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk584<F: Float>(t1697: F, t3222: F, t10: F, t427: F, t474: F, t426: F, t156: F, t1682: F, t259: F, t47: F, t1558: F, t348: F) -> (F, F, F, F, F, F, F, F) {
    let t3223 = t1697 * t3222;
    let t3224 = t10 * t3223;
    let t3227 = t474 * t427;
    let t3228 = t426 * t3227;
    let t3230 = t156 * t1682;
    let t3231 = t426 * t3230;
    let t3234 = F::new(1.0) / t47 / t259;
    let t3237 = t1558 * t348;
    (t3223, t3224, t3227, t3228, t3230, t3231, t3234, t3237)
}
