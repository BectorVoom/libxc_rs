//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1271/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1271(t2480: f64, t3216: f64, t439: f64, t1426: f64, t6244: f64, t2485: f64, t3177: f64, t1420: f64, t6250: f64, t10255: f64, t2484: f64, t12908: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16724 = t439 * t3216 * t2480 / 45.0_f64;
    let t16727 = 2.0_f64 / 45.0_f64 * t439 * t1426 * t6244;
    let t16729 = t3177 * t2485 / 27.0_f64;
    let t16731 = 2.0_f64 / 27.0_f64 * t1420 * t6250;
    let t16734 = t439 * t10255 * t2484 / 27.0_f64;
    let t16735 = 8.0_f64 / 45.0_f64 * t12908;
    (t16724, t16727, t16729, t16731, t16734, t16735)
}
