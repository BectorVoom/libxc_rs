//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 915/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk915(t3868: f64, t571: f64, t9678: f64, t3893: f64, t529: f64, t3802: f64, t3846: f64, t519: f64, t3412: f64, t3859: f64, t3482: f64, t5237: f64) -> (f64, f64, f64, f64, f64) {
    let t9680 = t571 * t9678 * t3868;
    let t9700 = t3893 * t529;
    let t9711 = t519 * t3802 * t3846;
    let t9714 = t519 * t3859 * t3412;
    let t9718 = t519 * t5237 * t3482;
    (t9680, t9700, t9711, t9714, t9718)
}
