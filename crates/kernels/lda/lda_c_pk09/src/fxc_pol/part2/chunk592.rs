//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 592/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk592<F: Float>(t5646: F, t1315: F, t5308: F, t1402: F, t5031: F, t1287: F, t365: F, t1342: F, t5081: F, t1280: F, t1435: F, t372: F, t4977: F, t310: F, t1337: F, t1311: F, t4998: F) -> (F, F, F, F, F, F, F, F) {
    let t5647 = 1.0 / t5646;
    let t5654 = t1315 * t5308;
    let t5658 = t1402 * t5031;
    let t5659 = t5658 * t1287;
    let t5664 = t365 * t5031;
    let t5670 = t1342 * t5081;
    let t5672 = t1280 * t1435;
    let t5674 = t372 * t4977;
    let t5675 = t310 * t5674;
    let t5677 = 0.04115066352984959 * t1337 * t5675;
    let t5679 = 1.2536914064583544 * t1311 * t4998;
    (t5647, t5654, t5659, t5664, t5670, t5672, t5677, t5679)
}
