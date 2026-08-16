//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 895/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk895(t1526: f64, t1527: f64, t15567: f64, t15568: f64, t15575: f64, t3088: f64, t342: f64, t343: f64, t38341: f64, t38355: f64, t38357: f64, t38366: f64, t38369: f64, t72: f64, t7712: f64, t7736: f64, t7765: f64, t7789: f64, t7807: f64, t7829: f64, t8183: f64) -> f64 {
    let t38375 = -t38341 / 4.0_f64 + t1526 * t1527 * t7829 / 2.0_f64 - t1526 * t1527 * t7712 * t7765 / 2.0_f64 + t15567 * t15575 * t7789 / 2.0_f64 - t38355 + 2.0_f64 * t7736 + 2.0_f64 / 3.0_f64 * t1526 * t3088 * t38357 * t7765 - t15567 * t15568 * t7807 / 3.0_f64 + t38366 / 6.0_f64 + t38369 / 6.0_f64 - t342 * t343 * t72 * t8183 / 4.0_f64;
    t38375
}
