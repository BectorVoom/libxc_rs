//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 642/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk642(t1423: f64, t1894: f64, t2022: f64, t591: f64, t2026: f64, t1680: f64, t872: f64, t1696: f64, t794: f64, t208: f64, t213: f64, t2025: f64, t97: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5363 = 4.0_f64 / 135.0_f64 * t1423 * t1894;
    let t5369 = 4.0_f64 / 9.0_f64 * t2022 * t591;
    let t5370 = t2026 * t591;
    let t5372 = t872 * t1680;
    let t5374 = t794 * t1696;
    let t5375 = t5374 * t208;
    let t5376 = t5375 * t213;
    let t5378 = t2025 * t97;
    (t5363, t5369, t5370, t5372, t5374, t5375, t5376, t5378)
}
