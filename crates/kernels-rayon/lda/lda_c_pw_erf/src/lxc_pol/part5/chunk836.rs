//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 836/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk836(t203: f64, t7674: f64, t184: f64, t221: f64, t2334: f64, t5404: f64, t1319: f64, t1318: f64, t6426: f64, t739: f64, t3806: f64, t519: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7675 = t203 * t7674;
    let t7676 = t7675 * t184;
    let t7678 = 2.0_f64 / 15.0_f64 * t7676 * t221;
    let t7679 = t5404 * t2334;
    let t7680 = t1319 * t7679;
    let t7682 = 16.0_f64 / 15.0_f64 * t1318 * t7680;
    let t7683 = t6426 * t739;
    let t7684 = t3806 * t7683;
    let t7686 = 8.0_f64 / 15.0_f64 * t519 * t7684;
    (t7675, t7676, t7678, t7679, t7680, t7682, t7683, t7684, t7686)
}
