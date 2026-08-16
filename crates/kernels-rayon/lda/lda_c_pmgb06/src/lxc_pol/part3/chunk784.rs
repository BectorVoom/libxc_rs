//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 784/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk784(t208: f64, t5381: f64, t1798: f64, t579: f64, t213: f64, t5342: f64, t5347: f64, t5349: f64, t5352: f64, t5354: f64, t5356: f64, t5361: f64, t5363: f64, t5367: f64, t5369: f64, t5370: f64, t5372: f64, t5376: f64, t5379: f64) -> (f64, f64, f64, f64) {
    let t5382 = t5381 * t208;
    let t5385 = t1798 * t579;
    let t5386 = t5385 * t208;
    let t5388 = 2.0_f64 / 3.0_f64 * t5386 * t213;
    let t5389 = -t5342 + t5347 - t5349 + t5352 + t5354 - t5356 + t5361 - t5363 + t5367 + t5369 + 4.0_f64 / 9.0_f64 * t5370 - 2.0_f64 / 27.0_f64 * t5372 + t5376 / 3.0_f64 + 0.12155555555555556_f64 * t5379 + t5382 * t213 / 3.0_f64 + t5388;
    (t5382, t5385, t5386, t5389)
}
