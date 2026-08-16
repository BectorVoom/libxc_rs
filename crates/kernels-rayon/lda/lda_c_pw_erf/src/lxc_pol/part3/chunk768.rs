//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 768/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk768(t4995: f64, t5028: f64, t582: f64, t186: f64, t211: f64, t2072: f64, t2104: f64, t1284: f64, t1386: f64, t2120: f64, t1287: f64, t209: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5029 = t4995 + t5028;
    let t5030 = t582 * t5029;
    let t5031 = t186 * t5030;
    let t5033 = 2.0_f64 / 15.0_f64 * t211 * t5031;
    let t5035 = 8.0_f64 / 15.0_f64 * t2104 * t2072;
    let t5037 = 8.0_f64 / 15.0_f64 * t1284 * t2072;
    let t5039 = 16.0_f64 / 45.0_f64 * t2120 * t1386;
    let t5040 = t1287 * t209;
    (t5029, t5030, t5031, t5033, t5035, t5037, t5039, t5040)
}
