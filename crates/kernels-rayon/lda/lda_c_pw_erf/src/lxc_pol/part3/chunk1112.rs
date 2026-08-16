//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1112/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1112(t4753: f64, t4895: f64, t3416: f64, t1318: f64, t9436: f64, t518: f64, t5400: f64, t577: f64, t10015: f64, t4484: f64, t1328: f64, t2098: f64, t3965: f64, t3966: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13009 = 8.0_f64 / 5.0_f64 * t4753 * t4895;
    let t13011 = 8.0_f64 / 5.0_f64 * t3416 * t4895;
    let t13013 = 8.0_f64 / 15.0_f64 * t1318 * t9436;
    let t13014 = t5400 * t518;
    let t13016 = 8.0_f64 / 15.0_f64 * t13014 * t577;
    let t13018 = 16.0_f64 / 15.0_f64 * t10015 * t4484;
    let t13022 = 16.0_f64 / 15.0_f64 * t3965 * t3966 * t2098 * t1328;
    (t13009, t13011, t13013, t13016, t13018, t13022)
}
