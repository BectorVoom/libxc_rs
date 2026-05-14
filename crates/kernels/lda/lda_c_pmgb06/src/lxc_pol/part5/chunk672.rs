//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 672/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk672<F: Float>(t2605: F, t435: F, t132: F, t337: F, t6560: F, t5069: F, t5068: F, t5139: F, t5138: F, t1593: F, t443: F, t760: F, t822: F, t477: F, t5077: F, t332: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t6626 = t435 * t2605;
    let t6627 = t132 * t6626;
    let t6628 = 2.0 / 45.0 * t6627;
    let t6629 = t6560 * t337;
    let t6630 = t5069 * t6629;
    let t6632 = 4.0 / 45.0 * t5068 * t6630;
    let t6633 = t5139 * t6629;
    let t6635 = 2.0 / 27.0 * t5138 * t6633;
    let t6636 = t1593 * t443;
    let t6637 = t760 * t822;
    let t6638 = t6637 * t477;
    let t6639 = t6636 * t6638;
    let t6641 = 4.0 / 45.0 * t5077 * t6639;
    let t6642 = t6637 * t332;
    (t6626, t6627, t6628, t6630, t6632, t6633, t6635, t6636, t6637, t6639, t6641, t6642)
}
