//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1106/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1106(t2002: f64, t3012: f64, t1420: f64, t5203: f64, t2007: f64, t3177: f64, t1511: f64, t1980: f64, t2012: f64, t5171: f64, t439: f64, t805: f64, t9373: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13158 = 2.0_f64 / 15.0_f64 * t2002 * t3012;
    let t13160 = 2.0_f64 / 5.0_f64 * t1420 * t5203;
    let t13162 = t3177 * t2007 / 15.0_f64;
    let t13165 = 2.0_f64 / 15.0_f64 * t1511 * t1980 * t2012;
    let t13167 = t1420 * t5171 / 15.0_f64;
    let t13170 = t439 * t9373 * t805 / 45.0_f64;
    (t13158, t13160, t13162, t13165, t13167, t13170)
}
