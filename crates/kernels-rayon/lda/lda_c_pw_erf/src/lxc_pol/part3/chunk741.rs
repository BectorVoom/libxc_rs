//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 741/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk741(t2035: f64, t3416: f64, t2002: f64, t1315: f64, t2171: f64, t2098: f64, t504: f64, t348: f64, t1313: f64, t519: f64, t2103: f64, t518: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4743 = 16.0_f64 / 45.0_f64 * t3416 * t2035;
    let t4745 = 16.0_f64 / 45.0_f64 * t3416 * t2002;
    let t4747 = 8.0_f64 / 45.0_f64 * t2171 * t1315;
    let t4748 = t2098 * t504;
    let t4749 = t4748 * t348;
    let t4750 = t1313 * t4749;
    let t4752 = 8.0_f64 / 45.0_f64 * t519 * t4750;
    let t4753 = t2103 * t518;
    (t4743, t4745, t4747, t4748, t4749, t4750, t4752, t4753)
}
