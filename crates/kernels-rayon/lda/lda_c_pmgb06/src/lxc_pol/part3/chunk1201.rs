//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1201/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1201(t12508: f64, t12511: f64, t12518: f64, t12524: f64, t12527: f64, t12534: f64, t12542: f64, t12545: f64, t12550: f64, t12553: f64, t12557: f64, t10691: f64, t10693: f64, t10696: f64, t10697: f64, t10699: f64, t12561: f64, t12566: f64, t12571: f64, t12574: f64, t12576: f64, t12579: f64, t12583: f64) -> (f64, f64) {
    let t14373 = -t12508 - t12511 - t12518 + t12524 - t12527 - t12534 + t12542 - t12545 + t12550 + t12553 + t12557;
    let t14378 = 4.0_f64 / 3.0_f64 * t10691 + 0.0033101111111111113_f64 * t10693 + t10696 + 8.0_f64 * t10697 + 12.0_f64 * t10699 + t12561 - t12566 + t12571 + t12574 + t12576 + t12579 + t12583;
    (t14373, t14378)
}
