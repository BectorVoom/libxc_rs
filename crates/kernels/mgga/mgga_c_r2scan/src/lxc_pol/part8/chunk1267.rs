//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1267/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1267<F: Float>(t3053: F, t551: F, t574: F, t6343: F, t7383: F, t8054: F, t7494: F, t9536: F, t2133: F, t2294: F, t8826: F, t6118: F, t8813: F, t7466: F, t7984: F, t6149: F, t9390: F) -> (F, F, F, F, F, F, F) {
    let t29298 = t574 * t551 * t6343 * t3053;
    let t29319 = t7383 * t8054;
    let t29354 = t7494 * t9536;
    let t29363 = t2133 * t2294 * t8826;
    let t29381 = t6118 * t8813;
    let t29392 = t7984 * t7466;
    let t29394 = t6149 * t9390;
    (t29298, t29319, t29354, t29363, t29381, t29392, t29394)
}
