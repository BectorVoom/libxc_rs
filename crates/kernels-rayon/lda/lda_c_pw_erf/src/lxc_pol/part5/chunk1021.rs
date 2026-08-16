//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1021/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1021(t2114: f64, t6220: f64, t1298: f64, t493: f64, t514: f64, t6591: f64, t1234: f64, t2407: f64, t1518: f64, t211: f64, t2467: f64, t2076: f64, t5175: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17058 = t2114 * t6220;
    let t17060 = t1298 * t6220;
    let t17063 = t493 * t514 * t6591;
    let t17079 = t2407 * t1234;
    let t17102 = t211 * t1518 * t2467;
    let t17105 = t2076 * t5175;
    (t17058, t17060, t17063, t17079, t17102, t17105)
}
