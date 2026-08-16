//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1154/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1154(t13775: f64, t2960: f64, t3010: f64, t3098: f64, t439: f64, t822: f64, t1447: f64, t5277: f64, t10216: f64, t176: f64, t1821: f64, t493: f64) -> (f64, f64, f64, f64) {
    let t13776 = 2.0_f64 / 27.0_f64 * t13775;
    let t13781 = 2.0_f64 / 9.0_f64 * t439 * t2960 * t822 * t3098 * t3010;
    let t13782 = t1447 * t5277;
    let t13783 = 2.0_f64 / 45.0_f64 * t13782;
    let t13787 = t493 * t10216 * t176 * t1821 / 9.0_f64;
    (t13776, t13781, t13783, t13787)
}
