//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 642/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk642<F: Float>(t1423: F, t1894: F, t2022: F, t591: F, t2026: F, t1680: F, t872: F, t1696: F, t794: F, t208: F, t213: F, t2025: F, t97: F) -> (F, F, F, F, F, F, F, F) {
    let t5363 = F::cast_from(4.0_f64) / F::cast_from(135.0_f64) * t1423 * t1894;
    let t5369 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t2022 * t591;
    let t5370 = t2026 * t591;
    let t5372 = t872 * t1680;
    let t5374 = t794 * t1696;
    let t5375 = t5374 * t208;
    let t5376 = t5375 * t213;
    let t5378 = t2025 * t97;
    (t5363, t5369, t5370, t5372, t5374, t5375, t5376, t5378)
}
