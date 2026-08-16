//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1146/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1146(t10326: f64, t10350: f64, t10361: f64, t10654: f64, t1318: f64, t2001: f64, t3854: f64, t5405: f64, t2171: f64, t3808: f64, t1472: f64, t4788: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13415 = 16.0_f64 / 15.0_f64 * t10326;
    let t13416 = 16.0_f64 / 45.0_f64 * t10350;
    let t13417 = 16.0_f64 / 135.0_f64 * t10361;
    let t13419 = t1318 * t10654 * t2001;
    let t13420 = 16.0_f64 / 135.0_f64 * t13419;
    let t13422 = t1318 * t3854 * t5405;
    let t13423 = 32.0_f64 / 45.0_f64 * t13422;
    let t13425 = 8.0_f64 / 15.0_f64 * t2171 * t3808;
    let t13426 = t1472 * t4788;
    (t13415, t13416, t13417, t13420, t13423, t13425, t13426)
}
