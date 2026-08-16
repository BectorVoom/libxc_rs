//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1150/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1150(t3576: f64, t822: f64, t10371: f64, t10403: f64, t1446: f64, t5360: f64, t5397: f64, t1318: f64, t1466: f64, t2156: f64, t3563: f64, t3770: f64, t4763: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13464 = t822 * t3576;
    let t13465 = 4.0_f64 / 15.0_f64 * t13464;
    let t13466 = 16.0_f64 / 45.0_f64 * t10371;
    let t13467 = 32.0_f64 / 45.0_f64 * t10403;
    let t13469 = 4.0_f64 / 5.0_f64 * t1446 * t5360;
    let t13470 = t1446 * t5397;
    let t13471 = 16.0_f64 / 15.0_f64 * t13470;
    let t13475 = 4.0_f64 / 15.0_f64 * t1318 * t1466 * t2156 * t3563;
    let t13477 = 4.0_f64 / 5.0_f64 * t4763 * t3770;
    (t13465, t13466, t13467, t13469, t13471, t13475, t13477)
}
