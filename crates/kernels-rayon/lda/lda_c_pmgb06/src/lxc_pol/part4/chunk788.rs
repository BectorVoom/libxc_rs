//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 788/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk788(t1897: f64, t4672: f64, t439: f64, t1901: f64, t4650: f64, t4668: f64, t2010: f64, t1420: f64, t1902: f64, t153: f64, t3279: f64, t1859: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5242 = t1897 * t4672;
    let t5244 = 2.0_f64 / 45.0_f64 * t439 * t5242;
    let t5245 = t1901 * t4650;
    let t5247 = 2.0_f64 / 9.0_f64 * t439 * t5245;
    let t5248 = t1897 * t4668;
    let t5250 = 8.0_f64 / 45.0_f64 * t2010 * t5248;
    let t5252 = 2.0_f64 / 27.0_f64 * t1420 * t1902;
    let t5253 = t3279 * t153;
    let t5254 = t5253 * t1859;
    (t5242, t5244, t5245, t5247, t5248, t5250, t5252, t5253, t5254)
}
