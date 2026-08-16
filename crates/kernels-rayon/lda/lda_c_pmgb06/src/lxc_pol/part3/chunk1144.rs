//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1144/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1144(t132: f64, t13554: f64, t13591: f64, t13623: f64, t13662: f64, t137: f64, t465: f64, t13508: f64, t13510: f64, t13512: f64, t13514: f64, t13516: f64, t13519: f64, t13521: f64, t13525: f64, t13527: f64, t13529: f64, t13530: f64) -> (f64, f64) {
    let t13668 = t132 * t137 * t465 * (t13554 + t13591 + t13623 + t13662) / 30.0_f64;
    let t13669 = t13508 + t13510 + t13512 + t13514 + t13516 + t13519 + t13521 + t13525 - t13527 - t13529 - t13530 - t13668;
    (t13668, t13669)
}
