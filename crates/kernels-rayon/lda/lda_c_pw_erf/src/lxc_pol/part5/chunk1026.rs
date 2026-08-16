//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1026/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1026(t2120: f64, t4568: f64, t1518: f64, t2504: f64, t493: f64, t3899: f64, t571: f64, t6969: f64, t6973: f64, t1294: f64, t2402: f64, t5175: f64, t6875: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17458 = t2120 * t4568;
    let t17461 = t493 * t1518 * t2504;
    let t17505 = t571 * t3899 * t6969;
    let t17508 = t571 * t3899 * t6973;
    let t17548 = t2402 * t1294;
    let t17550 = t6875 * t5175;
    (t17458, t17461, t17505, t17508, t17548, t17550)
}
