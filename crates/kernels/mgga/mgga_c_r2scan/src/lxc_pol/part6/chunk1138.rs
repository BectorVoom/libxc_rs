//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1138/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1138<F: Float>(t1592: F, t1632: F, t551: F, t6428: F, t1541: F, t2182: F, t2168: F, t6448: F, t6362: F, t6364: F, t489: F, t5134: F, t548: F, t6218: F, t6219: F, t6278: F, t7494: F) -> (F, F, F, F, F, F, F, F) {
    let t20696 = t1592 * t551 * t1632 * t6428;
    let t20698 = t2182 * t1541;
    let t20705 = t6448 * t2168;
    let t20710 = t6362 * t551 * t1632 * t6364;
    let t20720 = t5134 * t489;
    let t20721 = t20720 * t548;
    let t20729 = t6218 * t551 * t1632 * t6219;
    let t20731 = t7494 * t6278;
    (t20696, t20698, t20705, t20710, t20720, t20721, t20729, t20731)
}
