//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 932/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk932(t3933: f64, t656: f64, t1: f64, t3921: f64, t4166: f64, t119: f64, t1426: f64, t3920: f64, t19: f64, t2877: f64, t646: f64, t732: f64) -> (f64, f64, f64, f64) {
    let t11063 = 8.0_f64 / 9.0_f64 * t3933 * t656;
    let t11065 = t4166 * t1 * t3921;
    let t11069 = 0.006061752703703704_f64 * t3920 * t119 * t1426;
    let t11073 = 0.0002763148940771605_f64 * t2877 * t19 * t732 * t646;
    (t11063, t11065, t11069, t11073)
}
