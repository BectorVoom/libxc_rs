//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1208/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1208(t14240: f64, t4666: f64, t571: f64, t4680: f64, t4794: f64, t3394: f64, t4738: f64, t3399: f64, t2011: f64, t3727: f64, t10605: f64, t1944: f64, t219: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14242 = t571 * t14240 * t4666;
    let t14243 = 64.0_f64 / 81.0_f64 * t14242;
    let t14245 = t571 * t4794 * t4680;
    let t14246 = 8.0_f64 / 27.0_f64 * t14245;
    let t14248 = 8.0_f64 / 15.0_f64 * t4738 * t3394;
    let t14250 = 8.0_f64 / 9.0_f64 * t4738 * t3399;
    let t14252 = 4.0_f64 / 15.0_f64 * t3727 * t2011;
    let t14255 = t571 * t10605 * t219 * t1944;
    (t14243, t14246, t14248, t14250, t14252, t14255)
}
