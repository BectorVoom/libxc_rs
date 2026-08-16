//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1272/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1272(t12913: f64, t12915: f64, t12917: f64, t12919: f64, t12519: f64, t16527: f64, t5083: f64, t4790: f64, t831: f64, t12043: f64, t1981: f64, t496: f64, t851: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16736 = 8.0_f64 / 135.0_f64 * t12913;
    let t16737 = 8.0_f64 / 135.0_f64 * t12915;
    let t16738 = 4.0_f64 / 135.0_f64 * t12917;
    let t16739 = 4.0_f64 / 81.0_f64 * t12919;
    let t16742 = 8.0_f64 / 27.0_f64 * t5083 * t12519 * t16527;
    let t16743 = t831 * t4790;
    let t16744 = 4.0_f64 / 45.0_f64 * t16743;
    let t16748 = 4.0_f64 / 45.0_f64 * t1981 * t496 * t12043 * t851;
    (t16736, t16737, t16738, t16739, t16742, t16744, t16748)
}
