//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 989/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk989<F: Float>(t10271: F, t41962: F, t89: F, t295: F, t41446: F, t41448: F, t9716: F, t193: F, t2682: F, t2739: F, t7640: F, t2675: F, t9733: F, t43453: F, t43457: F, t43460: F, t43463: F, t43466: F, t43471: F, t43474: F, t43478: F, t43483: F, t43487: F, t43490: F) -> (F, F, F, F, F) {
    let t43493 = t89 * t41962 * t10271;
    let t43495 = t295 * t41446;
    let t43498 = t89 * t9716 * t43495 * t41448;
    let t43503 = t89 * t193 * t7640 * t2682 * t2739;
    let t43506 = t89 * t9733 * t2675;
    let t43508 = -8.0 / 3.0 * t43453 - 8.0 / 3.0 * t43457 + 4.0 / 27.0 * t43460 + 16.0 / 27.0 * t43463 + 8.0 / 9.0 * t43466 + 40.0 / 81.0 * t43471 - 20.0 / 27.0 * t43474 + 8.0 / 3.0 * t43478 - 80.0 / 243.0 * t43483 - t43487 / 9.0 - 16.0 / 27.0 * t43490 + 40.0 / 243.0 * t43493 + 40.0 / 27.0 * t43498 - 12.0 * t43503 - 8.0 / 27.0 * t43506;
    (t43493, t43498, t43503, t43506, t43508)
}
