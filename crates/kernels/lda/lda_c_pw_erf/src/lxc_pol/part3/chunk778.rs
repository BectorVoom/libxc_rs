//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 778/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk778<F: Float>(t3976: F, t5155: F, t593: F, t3974: F, t1333: F, t4574: F, t352: F, t1484: F, t219: F, t1351: F, t2066: F, t514: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t5157 = t3976 * t5155 * t593;
    let t5159 = F::new(16.0) / F::new(45.0) * t3974 * t5157;
    let t5160 = t4574 * t1333;
    let t5161 = t5155 * t352;
    let t5162 = t5160 * t5161;
    let t5164 = F::new(32.0) / F::new(45.0) * t3974 * t5162;
    let t5165 = t1484 * t219;
    let t5166 = t5165 * t1351;
    let t5167 = t5166 * t5161;
    let t5169 = F::new(16.0) / F::new(27.0) * t3974 * t5167;
    let t5170 = t514 * t2066;
    (t5157, t5159, t5160, t5162, t5164, t5165, t5166, t5167, t5169, t5170)
}
