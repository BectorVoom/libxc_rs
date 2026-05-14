//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 928/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk928<F: Float>(t27: F, t34: F, t1435: F, t5075: F, t5087: F, t1438: F, t1593: F, t1594: F, t332: F, t760: F, t5083: F, t12501: F, t5138: F, t5139: F, t3247: F, t5065: F, t5066: F) -> (F, F, F, F, F, F) {
    let t12514 = t27 * t34;
    let t12516 = t5075 * t12514 * t1435;
    let t12517 = t12516 * t5087;
    let t12518 = 4.0 / 27.0 * t12517;
    let t12519 = t1593 * t1438;
    let t12521 = t760 * t1594 * t332;
    let t12524 = 2.0 / 9.0 * t5083 * t12519 * t12521;
    let t12527 = t5138 * t5139 * t12501 / 9.0;
    let t12529 = t5065 * t5066 * t3247;
    (t12514, t12518, t12521, t12524, t12527, t12529)
}
