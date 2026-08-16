//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1149/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1149(t13080: f64, t4689: f64, t571: f64, t1124: f64, t1484: f64, t219: f64, t4676: f64, t494: f64, t542: f64, t3965: f64, t4490: f64, t505: f64) -> (f64, f64, f64) {
    let t13452 = t571 * t13080 * t4689;
    let t13453 = 16.0_f64 / 9.0_f64 * t13452;
    let t13455 = t1124 * t1484 * t219;
    let t13457 = t571 * t13455 * t4676;
    let t13458 = 40.0_f64 / 27.0_f64 * t13457;
    let t13459 = t494 * t542;
    let t13463 = 32.0_f64 / 15.0_f64 * t3965 * t4490 * t505 * t13459;
    (t13453, t13458, t13463)
}
