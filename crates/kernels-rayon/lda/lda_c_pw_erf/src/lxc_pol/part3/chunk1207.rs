//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1207/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1207(t1446: f64, t5247: f64, t11687: f64, t1991: f64, t519: f64, t3854: f64, t4693: f64, t571: f64, t4671: f64, t4794: f64, t10527: f64, t219: f64) -> (f64, f64, f64, f64, f64) {
    let t14230 = 4.0_f64 / 9.0_f64 * t1446 * t5247;
    let t14233 = 4.0_f64 / 27.0_f64 * t519 * t1991 * t11687;
    let t14235 = t571 * t3854 * t4693;
    let t14236 = 16.0_f64 / 45.0_f64 * t14235;
    let t14238 = t571 * t4794 * t4671;
    let t14239 = 16.0_f64 / 9.0_f64 * t14238;
    let t14240 = t10527 * t219;
    (t14230, t14233, t14236, t14239, t14240)
}
