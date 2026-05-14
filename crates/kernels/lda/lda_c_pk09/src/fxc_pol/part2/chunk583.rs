//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 583/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk583<F: Float>(t1345: F, t5081: F, t382: F, t5031: F, t5039: F, t5161: F, t5045: F, t5190: F, t5208: F, t5212: F, t5068: F, t1388: F, t1398: F, t747: F, t1383: F, t1435: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t5341 = t1345 * t5081;
    let t5343 = t382 * t5031;
    let t5348 = 0.821419393556371 * t5039;
    let t5352 = 5.333333333333333 * t5161;
    let t5361 = 0.5476129290375806 * t5045;
    let t5362 = 0.4444444444444444 * t5190;
    let t5367 = 4.0 * t5208;
    let t5368 = 4.0 * t5212;
    let t5370 = 0.18253764301252687 * t5068;
    let t5385 = t1388 * t747 * t1398;
    let t5389 = t1383 * t1435;
    (t5341, t5343, t5348, t5352, t5361, t5362, t5367, t5368, t5370, t5385, t5389)
}
