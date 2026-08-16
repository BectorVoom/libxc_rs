//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1118/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1118(t20424: f64, t5068: f64, t529: f64, t6559: f64, t9890: f64, t2043: f64, t2592: f64, t2066: f64, t1420: f64, t7696: f64, t439: f64, t5197: f64, t7695: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20435 = 2.0_f64 / 15.0_f64 * t5068 * t6559 * t20424 * t529;
    let t20436 = 4.0_f64 / 405.0_f64 * t9890;
    let t20438 = t2592 * t2043 / 10.0_f64;
    let t20440 = t2592 * t2066 / 10.0_f64;
    let t20442 = t1420 * t7696 / 5.0_f64;
    let t20445 = t439 * t5197 * t7695 / 5.0_f64;
    (t20435, t20436, t20438, t20440, t20442, t20445)
}
