//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1062/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1062<F: Float>(t1183: F, t1798: F, t297: F, t301: F, t2789: F, t794: F, t1767: F, t1770: F, t419: F, t1186: F, t5899: F, t1193: F, t4001: F, t4299: F) -> (F, F, F, F, F) {
    let t11600 = t297 * t1798 * t1183 * t301;
    let t11604 = t297 * t794 * t2789 * t301;
    let t11608 = t1767 * t1798 * t419 * t1770;
    let t11611 = t5899 * t1186 * t1770;
    let t11615 = t4001 * t794 * t1193 * t4299;
    (t11600, t11604, t11608, t11611, t11615)
}
