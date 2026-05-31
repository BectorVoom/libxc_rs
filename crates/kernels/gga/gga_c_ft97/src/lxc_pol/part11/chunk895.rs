//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 895/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk895<F: Float>(t1526: F, t1527: F, t15567: F, t15568: F, t15575: F, t3088: F, t342: F, t343: F, t38341: F, t38355: F, t38357: F, t38366: F, t38369: F, t72: F, t7712: F, t7736: F, t7765: F, t7789: F, t7807: F, t7829: F, t8183: F) -> F {
    let t38375 = -t38341 / F::cast_from(4.0_f64) + t1526 * t1527 * t7829 / F::cast_from(2.0_f64) - t1526 * t1527 * t7712 * t7765 / F::cast_from(2.0_f64) + t15567 * t15575 * t7789 / F::cast_from(2.0_f64) - t38355 + F::cast_from(2.0_f64) * t7736 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1526 * t3088 * t38357 * t7765 - t15567 * t15568 * t7807 / F::cast_from(3.0_f64) + t38366 / F::cast_from(6.0_f64) + t38369 / F::cast_from(6.0_f64) - t342 * t343 * t72 * t8183 / F::cast_from(4.0_f64);
    t38375
}
