//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1152/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1152(t10474: f64, t2168: f64, t4588: f64, t518: f64, t525: f64, t12881: f64, t3899: f64, t4929: f64, t571: f64, t2146: f64, t3748: f64, t3752: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13486 = 4.0_f64 / 5.0_f64 * t10474 * t2168;
    let t13487 = t4588 * t518;
    let t13489 = 8.0_f64 / 15.0_f64 * t13487 * t525;
    let t13491 = 4.0_f64 / 5.0_f64 * t12881 * t2168;
    let t13493 = t571 * t3899 * t4929;
    let t13494 = 8.0_f64 / 5.0_f64 * t13493;
    let t13495 = t2146 * t3748;
    let t13496 = 16.0_f64 / 45.0_f64 * t13495;
    let t13498 = 8.0_f64 / 15.0_f64 * t2146 * t3752;
    (t13486, t13489, t13491, t13494, t13496, t13498)
}
