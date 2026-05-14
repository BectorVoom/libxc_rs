//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1119/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1119<F: Float>(t106: F, t488: F, t6336: F, t6518: F, t1584: F, t6345: F, t1551: F, t551: F, t574: F, t6343: F, t1591: F, t6474: F, t1593: F, t6212: F, t6211: F, t494: F, t6127: F) -> (F, F, F, F, F, F, F) {
    let t20200 = 1.0 / t488 / t106;
    let t20229 = t6518 * t6336;
    let t20231 = t1584 * t6345;
    let t20235 = t574 * t551 * t6343 * t1551;
    let t20237 = t1591 * t6474;
    let t20238 = t6212 * t1593;
    let t20240 = t20237 * t6211 * t20238;
    let t20242 = t6127 * t494;
    (t20200, t20229, t20231, t20235, t20237, t20240, t20242)
}
