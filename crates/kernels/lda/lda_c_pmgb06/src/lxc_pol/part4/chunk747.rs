//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 747/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk747<F: Float>(t208: F, t5374: F, t213: F, t2025: F, t97: F, t588: F, t205: F, t4463: F, t1798: F, t579: F, t5342: F, t5347: F, t5349: F, t5352: F, t5354: F, t5356: F, t5361: F, t5363: F, t5367: F, t5369: F, t5370: F, t5372: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t5375 = t5374 * t208;
    let t5376 = t5375 * t213;
    let t5378 = t2025 * t97;
    let t5379 = t5378 * t588;
    let t5381 = t4463 * t205;
    let t5382 = t5381 * t208;
    let t5385 = t1798 * t579;
    let t5386 = t5385 * t208;
    let t5388 = 2.0 / 3.0 * t5386 * t213;
    let t5389 = -t5342 + t5347 - t5349 + t5352 + t5354 - t5356 + t5361 - t5363 + t5367 + t5369 + 4.0 / 9.0 * t5370 - 2.0 / 27.0 * t5372 + t5376 / 3.0 + 0.12155555555555556 * t5379 + t5382 * t213 / 3.0 + t5388;
    (t5375, t5376, t5378, t5379, t5381, t5382, t5385, t5386, t5388, t5389)
}
