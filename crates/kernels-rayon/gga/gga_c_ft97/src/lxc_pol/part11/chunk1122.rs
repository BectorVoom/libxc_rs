//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1122/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1122(t41448: f64, t43495: f64, t89: f64, t9716: f64, t193: f64, t2682: f64, t2739: f64, t7640: f64, t2675: f64, t9733: f64, t43453: f64, t43457: f64, t43460: f64, t43463: f64, t43466: f64, t43471: f64, t43474: f64, t43478: f64, t43483: f64, t43487: f64, t43490: f64, t43493: f64) -> (f64, f64, f64, f64) {
    let t43498 = t89 * t9716 * t43495 * t41448;
    let t43503 = t89 * t193 * t7640 * t2682 * t2739;
    let t43506 = t89 * t9733 * t2675;
    let t43508 = -8.0_f64 / 3.0_f64 * t43453 - 8.0_f64 / 3.0_f64 * t43457 + 4.0_f64 / 27.0_f64 * t43460 + 16.0_f64 / 27.0_f64 * t43463 + 8.0_f64 / 9.0_f64 * t43466 + 40.0_f64 / 81.0_f64 * t43471 - 20.0_f64 / 27.0_f64 * t43474 + 8.0_f64 / 3.0_f64 * t43478 - 80.0_f64 / 243.0_f64 * t43483 - t43487 / 9.0_f64 - 16.0_f64 / 27.0_f64 * t43490 + 40.0_f64 / 243.0_f64 * t43493 + 40.0_f64 / 27.0_f64 * t43498 - 12.0_f64 * t43503 - 8.0_f64 / 27.0_f64 * t43506;
    (t43498, t43503, t43506, t43508)
}
