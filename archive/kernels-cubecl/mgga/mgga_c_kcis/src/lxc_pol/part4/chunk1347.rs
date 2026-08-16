//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1347/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1347<F: Float>(t17346: F, t5910: F, t15860: F, t5904: F, t4292: F, t2061: F, t4287: F, t4286: F, t4266: F, t6016: F, t16665: F, t6028: F) -> (F, F, F, F, F) {
    let t17347 = t17346 * t5910;
    let t17349 = t5904 * t15860;
    let t17350 = t4292 * t17349;
    let t17352 = t2061 * t4287;
    let t17353 = t4286 * t17352;
    let t17355 = t6016 * t4266;
    let t17357 = t6028 * t16665;
    (t17347, t17350, t17353, t17355, t17357)
}
