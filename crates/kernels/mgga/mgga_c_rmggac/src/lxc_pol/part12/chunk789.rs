//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 789/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk789<F: Float>(t1591: F, t2039: F, t270: F, t638: F, t2338: F, t7323: F, t7324: F, t1327: F, t574: F, t640: F, t1243: F, t236: F, t3351: F, t618: F, t9210: F, t7248: F, t833: F) -> (F, F, F, F, F) {
    let t39338 = t638 * t2039 * t1591 * t270;
    let t39339 = 0.30487649791575028314e-3 * t39338;
    let t39341 = t7323 * t2338 * t7324;
    let t39345 = t7323 * t640 * t574 * t1327;
    let t39350 = t3351 * t9210 * t236 * t618 * t1243;
    let t39355 = t3351 * t7248 * t236 * t618 * t833;
    (t39339, t39341, t39345, t39350, t39355)
}
