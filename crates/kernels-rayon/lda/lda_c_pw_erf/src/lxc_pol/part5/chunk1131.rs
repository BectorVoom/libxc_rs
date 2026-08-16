//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1131/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1131(t2171: f64, t6489: f64, t6493: f64, t6353: f64, t6443: f64, t4804: f64, t7688: f64, t3794: f64, t1325: f64, t1326: f64, t6557: f64, t784: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t20947 = 8.0_f64 / 15.0_f64 * t2171 * t6489;
    let t20949 = 8.0_f64 / 5.0_f64 * t2171 * t6493;
    let t20951 = 8.0_f64 / 3.0_f64 * t2171 * t6353;
    let t20953 = 32.0_f64 / 15.0_f64 * t2171 * t6443;
    let t20955 = 8.0_f64 / 15.0_f64 * t4804 * t7688;
    let t20957 = 8.0_f64 / 15.0_f64 * t3794 * t7688;
    let t20961 = 8.0_f64 / 15.0_f64 * t1325 * t1326 * t6557 * t784;
    (t20947, t20949, t20951, t20953, t20955, t20957, t20961)
}
