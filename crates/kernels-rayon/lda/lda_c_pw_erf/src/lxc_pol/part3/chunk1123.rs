//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1123/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1123(t2153: f64, t3727: f64, t2146: f64, t4067: f64, t4052: f64, t10488: f64, t826: f64, t2140: f64, t3742: f64, t2143: f64, t3745: f64, t1401: f64, t1466: f64, t5029: f64, t571: f64, t593: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13137 = 8.0_f64 / 15.0_f64 * t3727 * t2153;
    let t13139 = 4.0_f64 / 45.0_f64 * t2146 * t4067;
    let t13141 = 32.0_f64 / 81.0_f64 * t2146 * t4052;
    let t13143 = 4.0_f64 / 45.0_f64 * t10488 * t826;
    let t13144 = t3742 * t2140;
    let t13145 = 16.0_f64 / 45.0_f64 * t13144;
    let t13146 = t3745 * t2143;
    let t13147 = 16.0_f64 / 45.0_f64 * t13146;
    let t13152 = 4.0_f64 / 5.0_f64 * t571 * t1466 * t1401 * t5029 * t593;
    (t13137, t13139, t13141, t13143, t13145, t13147, t13152)
}
