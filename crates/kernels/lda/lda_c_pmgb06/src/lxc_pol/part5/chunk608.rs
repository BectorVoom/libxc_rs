//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 608/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk608<F: Float>(t2026: F, t591: F, t1680: F, t872: F, t1696: F, t794: F, t208: F, t213: F, t2025: F, t97: F, t588: F, t1798: F, t579: F, t2021: F, t4876: F, t1450: F, t176: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t5370 = t2026 * t591;
    let t5372 = t872 * t1680;
    let t5374 = t794 * t1696;
    let t5375 = t5374 * t208;
    let t5376 = t5375 * t213;
    let t5378 = t2025 * t97;
    let t5379 = t5378 * t588;
    let t5385 = t1798 * t579;
    let t5386 = t5385 * t208;
    let t5388 = 2.0 / 3.0 * t5386 * t213;
    let t5391 = t2021 * t97;
    let t5393 = 0.12155555555555556 * t5391 * t588;
    let t5405 = 0.002518888888888889 * t4876;
    let t5447 = t1450 * t176;
    (t5370, t5372, t5374, t5375, t5376, t5378, t5379, t5385, t5386, t5388, t5391, t5393, t5405, t5447)
}
