//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 869/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk869<F: Float>(t1327: F, t574: F, t640: F, t7323: F, t1243: F, t236: F, t3351: F, t618: F, t9210: F, t7248: F, t833: F, t1614: F, t1971: F, t495: F, t511: F, t7230: F) -> (F, F, F, F) {
    let t39345 = t7323 * t640 * t574 * t1327;
    let t39350 = t3351 * t9210 * t236 * t618 * t1243;
    let t39355 = t3351 * t7248 * t236 * t618 * t833;
    let t39360 = t7230 * t1971 * t511 * t1614 * t495;
    (t39345, t39350, t39355, t39360)
}
