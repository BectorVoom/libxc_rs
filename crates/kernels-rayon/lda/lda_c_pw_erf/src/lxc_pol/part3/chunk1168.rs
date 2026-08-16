//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1168/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1168(t13607: f64, t13647: f64, t13692: f64, t13740: f64, t186: f64, t211: f64, t582: f64, t1513: f64, t2100: f64, t1284: f64, t4571: f64, t10011: f64, t4484: f64) -> (f64, f64, f64, f64) {
    let t13746 = 2.0_f64 / 15.0_f64 * t211 * t186 * t582 * (t13607 + t13647 + t13692 + t13740);
    let t13748 = 4.0_f64 / 5.0_f64 * t1513 * t2100;
    let t13749 = t1284 * t4571;
    let t13750 = 8.0_f64 / 45.0_f64 * t13749;
    let t13751 = t10011 * t4484;
    (t13746, t13748, t13750, t13751)
}
