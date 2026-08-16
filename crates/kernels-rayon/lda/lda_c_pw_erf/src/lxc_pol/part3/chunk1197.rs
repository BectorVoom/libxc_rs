//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1197/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1197(t14089: f64, t1: f64, t3: f64, t4713: f64, t604: f64, t1635: f64, t4537: f64, t1639: f64, t20: f64, t5794: f64, t1926: f64, t4196: f64) -> (f64, f64, f64, f64, f64) {
    let t14090 = 16.0_f64 / 405.0_f64 * t14089;
    let t14093 = t4713 * t1 * t3 * t604;
    let t14095 = t4537 * t1635;
    let t14096 = 0.6492624817418906_f64 * t14095;
    let t14098 = t5794 * t20 * t1639;
    let t14099 = 0.03354522822333102_f64 * t14098;
    let t14100 = t1926 * t4196;
    (t14090, t14093, t14096, t14099, t14100)
}
