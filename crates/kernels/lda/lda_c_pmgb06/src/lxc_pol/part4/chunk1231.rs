//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1231/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1231<F: Float>(t16213: F, t1898: F, t5220: F, t1902: F, t5211: F, t6478: F, t16187: F, t16189: F, t16190: F, t16192: F, t16195: F, t16199: F, t16201: F, t16204: F, t16207: F, t16210: F, t16212: F) -> (F, F, F, F, F) {
    let t16214 = F::new(8.0) / F::new(135.0) * t16213;
    let t16215 = t5220 * t1898;
    let t16216 = F::new(16.0) / F::new(135.0) * t16215;
    let t16217 = t5220 * t1902;
    let t16218 = F::new(8.0) / F::new(81.0) * t16217;
    let t16219 = t5211 * t6478;
    let t16220 = F::new(20.0) / F::new(81.0) * t16219;
    let t16221 = t16187 - t16189 - t16190 - t16192 - t16195 - t16199 - t16201 - t16204 - t16207 - t16210 - t16212 - t16214 - t16216 + t16218 - t16220;
    (t16214, t16216, t16218, t16220, t16221)
}
