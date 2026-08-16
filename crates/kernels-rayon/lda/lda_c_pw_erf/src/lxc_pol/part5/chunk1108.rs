//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1108/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1108(t1446: f64, t7621: f64, t20007: f64, t519: f64, t522: f64, t523: f64, t7625: f64, t2554: f64, t5327: f64, t15521: f64, t15525: f64, t15538: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t20670 = 4.0_f64 / 45.0_f64 * t1446 * t7621;
    let t20674 = 4.0_f64 / 45.0_f64 * t519 * t522 * t523 * t20007;
    let t20676 = 32.0_f64 / 81.0_f64 * t1446 * t7625;
    let t20678 = 4.0_f64 / 9.0_f64 * t5327 * t2554;
    let t20679 = 8.0_f64 / 81.0_f64 * t15521;
    let t20680 = 16.0_f64 / 15.0_f64 * t15525;
    let t20681 = 8.0_f64 / 45.0_f64 * t15538;
    (t20670, t20674, t20676, t20678, t20679, t20680, t20681)
}
