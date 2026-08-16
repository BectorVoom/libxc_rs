//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1102/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1102(t1455: f64, t5305: f64, t2002: f64, t3263: f64, t806: f64, t9365: f64, t1423: f64, t4609: f64, t5203: f64, t439: f64, t4608: f64, t5197: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13112 = t5305 * t1455 / 15.0_f64;
    let t13114 = 8.0_f64 / 81.0_f64 * t2002 * t3263;
    let t13116 = t9365 * t806 / 45.0_f64;
    let t13117 = t1423 * t4609;
    let t13118 = 2.0_f64 / 15.0_f64 * t13117;
    let t13119 = t1423 * t5203;
    let t13120 = 4.0_f64 / 15.0_f64 * t13119;
    let t13123 = t439 * t5197 * t4608 / 5.0_f64;
    (t13112, t13114, t13116, t13118, t13120, t13123)
}
