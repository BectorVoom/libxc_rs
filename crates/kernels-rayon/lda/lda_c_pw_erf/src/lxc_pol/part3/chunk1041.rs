//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1041/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1041(t1313: f64, t3545: f64, t519: f64, t789: f64, t10467: f64, t2030: f64, t1472: f64, t4838: f64, t1308: f64, t3655: f64, t571: f64, t816: f64) -> (f64, f64, f64, f64) {
    let t12194 = 4.0_f64 / 45.0_f64 * t519 * t1313 * t789 * t3545;
    let t12196 = t519 * t10467 * t2030;
    let t12197 = 8.0_f64 / 135.0_f64 * t12196;
    let t12199 = 4.0_f64 / 15.0_f64 * t1472 * t4838;
    let t12203 = 4.0_f64 / 45.0_f64 * t571 * t1308 * t816 * t3655;
    (t12194, t12197, t12199, t12203)
}
