//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 906/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk906(t5212: f64, t2064: f64, t2106: f64, t137: f64, t132: f64, t2090: f64, t831: f64, t2631: f64, t432: f64, t3306: f64, t5196: f64, t5207: f64, t5209: f64, t5215: f64, t5217: f64, t5219: f64, t5222: f64, t5304: f64, t5328: f64, t5330: f64, t5342: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6570 = 8.0_f64 / 135.0_f64 * t5212;
    let t6571 = t2106 * t2064;
    let t6572 = t137 * t6571;
    let t6574 = t132 * t6572 / 15.0_f64;
    let t6576 = t831 * t2090 / 15.0_f64;
    let t6578 = t432 * t2631 / 15.0_f64;
    let t6579 = t3306 / 135.0_f64;
    let t6580 = t5196 + t5207 + t5209 + t6570 + t5215 + t5217 + t5219 + t5222 - t5304 - t6574 - t6576 - t6578 - t6579 - t5328 - t5330 - t5342;
    (t6570, t6571, t6572, t6574, t6576, t6578, t6579, t6580)
}
