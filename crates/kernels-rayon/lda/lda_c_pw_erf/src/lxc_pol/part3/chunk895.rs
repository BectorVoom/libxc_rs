//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 895/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk895(t411: f64, t717: f64, t732: f64, t3257: f64, t1124: f64, t119: f64, t1657: f64, t8994: f64, t1690: f64, t415: f64, t8916: f64, t3273: f64, t8920: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8998 = t732 * t717 * t411;
    let t8999 = t3257 * t8998;
    let t9002 = t119 * t1124 * t411;
    let t9003 = t1657 * t9002;
    let t9005 = t1657 * t8994;
    let t9009 = t415 * t1690 * t8916;
    let t9011 = t3273 * t8920;
    (t8998, t8999, t9002, t9003, t9005, t9009, t9011)
}
