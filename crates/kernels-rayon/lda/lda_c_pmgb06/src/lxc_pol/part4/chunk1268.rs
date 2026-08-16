//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1268/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1268(t439: f64, t5225: f64, t6160: f64, t15445: f64, t1897: f64, t15353: f64, t15358: f64, t1901: f64, t1420: f64, t6419: f64, t5253: f64, t6165: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16662 = 4.0_f64 / 45.0_f64 * t439 * t5225 * t6160;
    let t16665 = 4.0_f64 / 45.0_f64 * t439 * t1897 * t15445;
    let t16668 = 2.0_f64 / 45.0_f64 * t439 * t1897 * t15353;
    let t16671 = 2.0_f64 / 9.0_f64 * t439 * t1901 * t15358;
    let t16673 = 2.0_f64 / 27.0_f64 * t1420 * t6419;
    let t16676 = 2.0_f64 / 27.0_f64 * t439 * t5253 * t6165;
    (t16662, t16665, t16668, t16671, t16673, t16676)
}
