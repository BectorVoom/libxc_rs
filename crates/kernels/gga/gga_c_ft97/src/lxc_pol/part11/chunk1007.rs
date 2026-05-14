//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1007/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1007<F: Float>(t43399: F, t43403: F, t43407: F, t43411: F, t43416: F, t43418: F, t43422: F, t43424: F, t43426: F, t43430: F, t43433: F, t43437: F, t43441: F, t43444: F, t43448: F, t43453: F, t43457: F, t43460: F, t43463: F, t43466: F, t43471: F, t43474: F, t43478: F, t43483: F, t43487: F, t43490: F, t43493: F, t43498: F, t43503: F, t43506: F) -> (F, F) {
    let t44096 = 2.0 * t43399 + 9.0 / 4.0 * t43403 - t43407 - 8.0 * t43411 + 4.0 / 3.0 * t43416 + 8.0 / 9.0 * t43418 + 4.0 / 3.0 * t43422 - 4.0 / 3.0 * t43424 - 4.0 / 3.0 * t43426 + 4.0 / 3.0 * t43430 + 8.0 / 3.0 * t43433 + 8.0 * t43437 + 2.0 * t43441 + 112.0 / 81.0 * t43444 - 2.0 / 3.0 * t43448;
    let t44113 = -8.0 * t43453 - 8.0 * t43457 + 4.0 / 9.0 * t43460 + 16.0 / 9.0 * t43463 + 8.0 / 3.0 * t43466 + 40.0 / 27.0 * t43471 - 20.0 / 9.0 * t43474 + 8.0 * t43478 - 80.0 / 81.0 * t43483 - t43487 / 3.0 - 16.0 / 9.0 * t43490 + 40.0 / 81.0 * t43493 + 40.0 / 9.0 * t43498 - 36.0 * t43503 - 8.0 / 9.0 * t43506;
    (t44096, t44113)
}
