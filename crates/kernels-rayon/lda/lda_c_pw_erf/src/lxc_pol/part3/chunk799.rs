//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 799/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk799(t2181: f64, t944: f64, t1440: f64, t2187: f64, t3787: f64, t519: f64, t1522: f64, t820: f64, t184: f64, t1333: f64, t811: f64, t951: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5393 = t2181 * t944;
    let t5394 = t1440 * t5393;
    let t5397 = t3787 * t2187;
    let t5399 = 16.0_f64 / 45.0_f64 * t519 * t5397;
    let t5400 = t1522 * t820;
    let t5401 = t5400 * t184;
    let t5404 = t811 * t1333;
    let t5405 = t5404 * t951;
    (t5393, t5394, t5397, t5399, t5400, t5401, t5405)
}
