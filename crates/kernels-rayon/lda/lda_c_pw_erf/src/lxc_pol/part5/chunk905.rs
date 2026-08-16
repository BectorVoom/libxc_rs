//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 905/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk905(t3257: f64, t8998: f64, t1124: f64, t119: f64, t411: f64, t1657: f64, t3267: f64, t1691: f64, t435: f64, t97: f64, t3338: f64, t440: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8999 = t3257 * t8998;
    let t9002 = t119 * t1124 * t411;
    let t9003 = t1657 * t9002;
    let t9017 = t3267 * t8998;
    let t9019 = t1691 * t9002;
    let t9037 = 1.0_f64 / t435 / t97;
    let t9059 = t440 * t3338;
    (t8999, t9003, t9017, t9019, t9037, t9059)
}
