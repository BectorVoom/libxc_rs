//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1141/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1141<F: Float>(t405: F, t6885: F, t4913: F, t6888: F, t4641: F, t6816: F, t350: F, t6805: F, t16073: F, t36: F, t506: F, t6824: F, t6821: F, t1464: F, t337: F, t5974: F) -> (F, F, F, F, F, F, F, F) {
    let t17127 = t405 * t6885;
    let t17129 = t4913 * t6888;
    let t17131 = t4641 * t6816;
    let t17133 = t350 * t6805;
    let t17136 = t36 * t506 * t16073;
    let t17138 = t350 * t6824;
    let t17140 = t350 * t6821;
    let t17143 = t1464 * t5974 * t337;
    (t17127, t17129, t17131, t17133, t17136, t17138, t17140, t17143)
}
