//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 542/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk542<F: Float>(t1553: F, t450: F, t1555: F, t1729: F, t452: F, t142: F, t1664: F, t455: F, t22: F, t342: F) -> (F, F, F, F, F, F) {
    let t2805 = t1553 * t450;
    let t2806 = t2805 * t1555;
    let t2809 = t1729 * t452;
    let t2810 = t142 * t1664;
    let t2811 = t455 * t2810;
    let t2817 = F::new(1.0) / t22 / t342;
    (t2805, t2806, t2809, t2810, t2811, t2817)
}
