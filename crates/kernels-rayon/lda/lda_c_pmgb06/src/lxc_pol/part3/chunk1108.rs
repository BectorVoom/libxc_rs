//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1108/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1108(t1981: f64, t835: f64, t1454: f64, t493: f64, t5312: f64, t1461: f64, t1835: f64, t1466: f64, t1989: f64, t3198: f64, t1444: f64, t4585: f64) -> (f64, f64, f64, f64, f64) {
    let t13177 = t1981 * t835;
    let t13178 = 8.0_f64 / 1215.0_f64 * t13177;
    let t13181 = t493 * t5312 * t1454 / 15.0_f64;
    let t13182 = t1461 * t1835;
    let t13185 = t493 * t13182 * t1466 / 9.0_f64;
    let t13187 = t3198 * t1989 / 15.0_f64;
    let t13189 = t1444 * t4585 / 15.0_f64;
    (t13178, t13181, t13185, t13187, t13189)
}
