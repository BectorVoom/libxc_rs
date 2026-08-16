//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1195/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1195(t181: f64, t184: f64, t3464: f64, t786: f64, t509: f64, t944: f64, t511: f64, t5129: f64, t4724: f64, t1397: f64, t5211: f64, t1498: f64, t2067: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14066 = 4.0_f64 / 15.0_f64 * t3464 * t181 * t184 * t786;
    let t14070 = 4.0_f64 / 5.0_f64 * t944 * t509 * t184 * t786;
    let t14072 = 2.0_f64 / 5.0_f64 * t511 * t5129;
    let t14074 = 8.0_f64 / 15.0_f64 * t511 * t4724;
    let t14075 = t5211 * t1397;
    let t14076 = 16.0_f64 / 15.0_f64 * t14075;
    let t14078 = 2.0_f64 / 5.0_f64 * t1498 * t2067;
    (t14066, t14070, t14072, t14074, t14076, t14078)
}
