//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 808/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk808<F: Float>(t342: F, t630: F, t7729: F, t344: F, t8639: F, t7800: F, t81: F, t1526: F, t7705: F, t7721: F, t1533: F, t2252: F, t1527: F, t15567: F, t15568: F, t15575: F, t3088: F, t343: F, t72: F, t7712: F, t7736: F, t7765: F, t7789: F, t7807: F, t7829: F, t8183: F) -> (F,) {
    let t38341 = t342 * t630 * t7729;
    let t38355 = 5.0 / 54.0 * t342 * t8639 * t344;
    let t38357 = t81 * t7800;
    let t38366 = t1526 * t7705 * t7721;
    let t38369 = t342 * t2252 * t1533;
    let t38375 = -t38341 / 4.0 + t1526 * t1527 * t7829 / 2.0 - t1526 * t1527 * t7712 * t7765 / 2.0 + t15567 * t15575 * t7789 / 2.0 - t38355 + 2.0 * t7736 + 2.0 / 3.0 * t1526 * t3088 * t38357 * t7765 - t15567 * t15568 * t7807 / 3.0 + t38366 / 6.0 + t38369 / 6.0 - t342 * t343 * t72 * t8183 / 4.0;
    (t38375,)
}
