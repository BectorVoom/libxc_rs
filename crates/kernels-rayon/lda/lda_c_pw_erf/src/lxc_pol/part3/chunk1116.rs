//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1116/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1116(t13054: f64, t1446: f64, t4750: f64, t1472: f64, t5286: f64, t1401: f64, t2151: f64, t1403: f64, t1954: f64, t571: f64, t519: f64, t5221: f64, t9723: f64) -> (f64, f64, f64, f64, f64) {
    let t13055 = 8.0_f64 / 45.0_f64 * t13054;
    let t13057 = 8.0_f64 / 15.0_f64 * t1446 * t4750;
    let t13059 = 8.0_f64 / 15.0_f64 * t1472 * t5286;
    let t13060 = t2151 * t1401;
    let t13064 = 16.0_f64 / 15.0_f64 * t571 * t13060 * t1954 * t1403;
    let t13066 = t519 * t9723 * t5221;
    (t13055, t13057, t13059, t13064, t13066)
}
