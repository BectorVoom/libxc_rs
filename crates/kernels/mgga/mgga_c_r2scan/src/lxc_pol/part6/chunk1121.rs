//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1121/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1121<F: Float>(t1632: F, t5066: F, t551: F, t574: F, t1600: F, t6355: F, t2168: F, t6217: F, t1554: F, t6212: F, t6209: F, t6211: F, t545: F, t6534: F, t1570: F, t560: F) -> (F, F, F, F, F, F) {
    let t20270 = t574 * t551 * t1632 * t5066;
    let t20279 = t1600 * t6355;
    let t20286 = t6217 * t2168;
    let t20294 = t6212 * t1554;
    let t20296 = t6209 * t6211 * t20294;
    let t20298 = t545 * t6534;
    let t20299 = t1570 * t560;
    (t20270, t20279, t20286, t20296, t20298, t20299)
}
