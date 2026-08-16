//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 890/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk890(t224: f64, t3145: f64, t3120: f64, t441: f64, t1455: f64, t3223: f64, t1467: f64, t1447: f64, t3174: f64, t3226: f64, t1423: f64, t3210: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9370 = t3145 * t224;
    let t9373 = t441 * t3120;
    let t9379 = t3223 * t1455;
    let t9381 = t3223 * t1467;
    let t9383 = t1447 * t3174;
    let t9385 = t3226 * t1467;
    let t9393 = t3226 * t1455;
    let t9395 = t1423 * t3210;
    (t9370, t9373, t9379, t9381, t9383, t9385, t9393, t9395)
}
