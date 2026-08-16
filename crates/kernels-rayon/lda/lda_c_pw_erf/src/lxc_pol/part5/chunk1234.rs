//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1234/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1234(t2550: f64, t5327: f64, t2171: f64, t6939: f64, t22189: f64, t22192: f64, t22194: f64, t22196: f64, t22200: f64, t22204: f64, t22207: f64, t22208: f64, t22210: f64, t22212: f64, t22214: f64) -> (f64, f64, f64) {
    let t22216 = 4.0_f64 / 15.0_f64 * t5327 * t2550;
    let t22218 = 4.0_f64 / 15.0_f64 * t2171 * t6939;
    let t22219 = -t22189 + t22192 - t22194 - t22196 - t22200 - t22204 + t22207 + t22208 + t22210 + t22212 + t22214 + t22216 + t22218;
    (t22216, t22218, t22219)
}
