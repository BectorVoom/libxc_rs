//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1198/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1198(t14347: f64, t1377: f64, t2345: f64, t97: f64, t27: f64, t545: f64, t5635: f64, t5638: f64, t5632: f64, t12230: f64, t12233: f64, t12235: f64, t12237: f64, t12240: f64, t12242: f64, t12244: f64) -> f64 {
    let t14348 = 0.03354522822333102_f64 * t14347;
    let t14350 = t2345 * t97 * t1377;
    let t14353 = t5635 * t27 * t545;
    let t14356 = t5638 * t27 * t545;
    let t14357 = 0.6492624817418906_f64 * t14356;
    let t14359 = t5632 * t27 * t545;
    let t14361 = t14348 + 0.03354522822333102_f64 * t14350 + 0.3246312408709453_f64 * t14353 + t14357 + 0.3246312408709453_f64 * t14359 + t12230 + t12233 + t12235 - t12237 + t12240 + t12242 - t12244;
    t14361
}
