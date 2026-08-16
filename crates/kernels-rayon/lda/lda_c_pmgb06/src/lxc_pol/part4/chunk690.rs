//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 690/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk690(t1179: f64, t206: f64, t4068: f64, t1830: f64, t2060: f64, t83: f64, t208: f64, t213: f64, t1697: f64, t97: f64, t588: f64, t1166: f64, t579: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4070 = 0.001515438175925926_f64 * t206 * t1179 * t4068;
    let t4075 = 0.1005925925925926_f64 * t1830 - 0.5007407407407407_f64 * t2060;
    let t4076 = t83 * t4075;
    let t4077 = t4076 * t208;
    let t4079 = t4077 * t213 / 3.0_f64;
    let t4080 = t1697 * t97;
    let t4082 = 0.18233333333333332_f64 * t4080 * t588;
    let t4087 = t1166 * t579;
    (t4070, t4075, t4076, t4077, t4079, t4080, t4082, t4087)
}
