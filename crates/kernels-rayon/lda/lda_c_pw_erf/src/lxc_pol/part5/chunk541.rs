//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 541/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk541(t168: f64, t2782: f64, t286: f64, t1553: f64, t450: f64, t1729: f64, t452: f64, t22: f64, t342: f64) -> (f64, f64, f64, f64, f64) {
    let t2783 = t168 * t2782;
    let t2785 = 0.19513566535229734_f64 * t2783 * t286;
    let t2805 = t1553 * t450;
    let t2809 = t1729 * t452;
    let t2817 = 1.0_f64 / t22 / t342;
    (t2783, t2785, t2805, t2809, t2817)
}
