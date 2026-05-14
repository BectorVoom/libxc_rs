//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 862/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk862<F: Float>(t1553: F, t277: F, t1569: F, t560: F, t360: F, t2597: F, t565: F) -> (F, F, F, F, F) {
    let t6133 = t277 * t1553;
    let t6134 = t1569 * t560;
    let t6135 = t6133 * t6134;
    let t6136 = t360 * t6135;
    let t6139 = t565 * t2597;
    (t6133, t6134, t6135, t6136, t6139)
}
