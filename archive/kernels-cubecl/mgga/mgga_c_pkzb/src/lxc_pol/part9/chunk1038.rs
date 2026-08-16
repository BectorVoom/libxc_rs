//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1038/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1038<F: Float>(t1306: F, t135: F, t2457: F, t273: F, t3286: F, t7920: F, t7922: F, t7924: F, t7926: F, t8006: F, t8008: F, t8011: F, t8015: F, t8019: F, t8023: F, t8025: F, t8027: F, t8030: F, t8034: F, t8241: F, t8243: F, t8291: F, t8563: F, t957: F) -> F {
    let t8567 = t135 * t273 * t8563 * t957 - t1306 * t2457 * t3286 - t7920 - t7922 + t7924 + t7926 + t8006 - t8008 - t8011 - t8015 + t8019 + t8023 - t8025 - t8027 - t8030 + t8034 + t8241 + t8243 - t8291;
    t8567
}
