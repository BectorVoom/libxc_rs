//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 718/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk718(t348: f64, t6426: f64, t3806: f64, t519: f64, t784: f64, t806: f64, t542: f64, t5289: f64, t1325: f64, t2031: f64, t2171: f64, t1987: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6427 = t6426 * t348;
    let t6428 = t3806 * t6427;
    let t6430 = 8.0_f64 / 45.0_f64 * t519 * t6428;
    let t6431 = t784 * t806;
    let t6432 = t6431 * t542;
    let t6433 = t5289 * t6432;
    let t6435 = 16.0_f64 / 15.0_f64 * t1325 * t6433;
    let t6437 = 8.0_f64 / 45.0_f64 * t2171 * t2031;
    let t6439 = 16.0_f64 / 45.0_f64 * t2171 * t1987;
    (t6427, t6428, t6430, t6431, t6432, t6433, t6435, t6437, t6439)
}
