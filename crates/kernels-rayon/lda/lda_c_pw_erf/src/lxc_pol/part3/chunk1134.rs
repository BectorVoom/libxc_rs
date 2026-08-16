//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1134/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1134(t1251: f64, t1313: f64, t2098: f64, t519: f64, t940: f64, t2954: f64, t3402: f64, t3476: f64, t806: f64, t1472: f64, t5279: f64, t2967: f64, t4665: f64) -> (f64, f64, f64, f64) {
    let t13282 = 8.0_f64 / 15.0_f64 * t519 * t1313 * t2098 * t1251 * t940;
    let t13287 = 8.0_f64 / 9.0_f64 * t519 * t3402 * t806 * t3476 * t2954;
    let t13289 = 8.0_f64 / 5.0_f64 * t1472 * t5279;
    let t13290 = t4665 * t2967;
    (t13282, t13287, t13289, t13290)
}
