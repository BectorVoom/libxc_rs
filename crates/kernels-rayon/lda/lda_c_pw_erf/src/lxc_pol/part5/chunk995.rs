//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 995/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk995(t1055: f64, t5967: f64, t402: f64, t6011: f64, t75: f64, t1051: f64, t390: f64, t40: f64, t19: f64, t729: f64, t7307: f64, t734: f64) -> (f64, f64, f64, f64, f64) {
    let t15341 = t5967 * t1055;
    let t15344 = t6011 * t75 * t402;
    let t15346 = t5967 * t1051;
    let t15349 = t40 * t6011 * t390;
    let t15413 = t7307 * t729 * t19 * t734;
    (t15341, t15344, t15346, t15349, t15413)
}
