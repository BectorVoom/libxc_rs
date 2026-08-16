//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1027/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1027(t197: f64, t3892: f64, t3518: f64, t11857: f64, t4488: f64, t1390: f64, t1440: f64, t5127: f64, t519: f64, t542: f64, t9359: f64, t9361: f64) -> (f64, f64, f64, f64, f64) {
    let t12030 = t3892 * t197;
    let t12031 = t12030 * t3518;
    let t12034 = 32.0_f64 / 27.0_f64 * t4488 * t12031 * t11857;
    let t12039 = 4.0_f64 / 5.0_f64 * t519 * t1440 * t1390 * t5127 * t542;
    let t12040 = 8.0_f64 / 15.0_f64 * t9359;
    let t12041 = 8.0_f64 / 15.0_f64 * t9361;
    (t12031, t12034, t12039, t12040, t12041)
}
