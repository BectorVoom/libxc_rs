//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 708/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk708(t3661: f64, t3664: f64, t3764: f64, t3785: f64, t4562: f64, t4565: f64, t4569: f64, t4572: f64, t2437: f64, t494: f64, t1326: f64, t1325: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6312 = 8.0_f64 / 135.0_f64 * t3661;
    let t6313 = 4.0_f64 / 135.0_f64 * t3664;
    let t6316 = 8.0_f64 / 405.0_f64 * t3764;
    let t6317 = 8.0_f64 / 405.0_f64 * t3785;
    let t6318 = 8.0_f64 / 135.0_f64 * t4562;
    let t6319 = 16.0_f64 / 135.0_f64 * t4565;
    let t6320 = 16.0_f64 / 45.0_f64 * t4569;
    let t6321 = 16.0_f64 / 135.0_f64 * t4572;
    let t6322 = t2437 * t494;
    let t6323 = t1326 * t6322;
    let t6325 = 8.0_f64 / 45.0_f64 * t1325 * t6323;
    (t6312, t6313, t6316, t6317, t6318, t6319, t6320, t6321, t6322, t6323, t6325)
}
