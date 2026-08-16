//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1065/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1065(t1278: f64, t3965: f64, t3967: f64, t5136: f64, t3704: f64, t3964: f64, t34: f64, t494: f64, t542: f64, t10015: f64, t5143: f64, t12446: f64, t5141: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12474 = 8.0_f64 / 15.0_f64 * t3965 * t3967 * t5136 * t1278;
    let t12475 = t3964 * t3704;
    let t12476 = t34 * t494;
    let t12480 = 32.0_f64 / 15.0_f64 * t12475 * t3967 * t12476 * t542;
    let t12482 = 32.0_f64 / 15.0_f64 * t10015 * t5143;
    let t12485 = 16.0_f64 / 15.0_f64 * t3965 * t5141 * t12446;
    (t12474, t12475, t12476, t12480, t12482, t12485)
}
