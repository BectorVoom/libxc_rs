//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1096/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1096(t4612: f64, t5211: f64, t1983: f64, t485: f64, t5210: f64, t5322: f64, t5499: f64, t806: f64, t9836: f64, t2007: f64, t3220: f64, t835: f64, t9271: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13196 = t5211 * t4612;
    let t13199 = t485 * t5210 * t1983;
    let t13201 = t5499 * t5322;
    let t13204 = t9836 * t806;
    let t13206 = t3220 * t2007;
    let t13211 = t9271 * t835;
    (t13196, t13199, t13201, t13204, t13206, t13211)
}
