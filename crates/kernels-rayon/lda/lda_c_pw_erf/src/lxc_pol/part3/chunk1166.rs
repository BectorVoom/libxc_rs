//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1166/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1166(t12289: f64, t1953: f64, t557: f64, t325: f64, t4694: f64, t4672: f64, t4606: f64, t4690: f64, t3618: f64, t817: f64, t1349: f64, t3609: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13720 = t1953 * t557 * t12289;
    let t13722 = t325 * t4694;
    let t13724 = t325 * t4672;
    let t13726 = t4606 * t4690;
    let t13729 = t1953 * t557 * t3618;
    let t13731 = t1953 * t817;
    let t13734 = t1953 * t1349 * t3609;
    (t13720, t13722, t13724, t13726, t13729, t13731, t13734)
}
