//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1309/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1309(t10419: f64, t10478: f64, t10485: f64, t1354: f64, t14098: f64, t14155: f64, t16677: f64, t16680: f64, t16683: f64, t16771: f64, t2493: f64, t2518: f64, t4869: f64, t4884: f64, t4888: f64, t4919: f64, t50450: f64, t57211: f64, t57246: f64, t57260: f64, t57327: f64, t57330: f64, t57332: f64, t57335: f64, t57337: f64, t7753: f64, t7759: f64, t7799: f64) -> f64 {
    let t57338 = 0.61523382126046769581e4_f64 * t7753 * t14098 * t4919 - 24.0_f64 * t10419 * t16677 + 0.38597619813444837568e3_f64 * t10478 * t16680 + t57246 - 0.19751789702565206229e-1_f64 * t57211 - t57260 + 36.0_f64 * t2518 * t4869 * t4884 - 0.11579285944033451271e4_f64 * t7759 * t4888 * t4884 - 8.0_f64 * t2493 * t16771 * t1354 + 0.1286587327114827919e3_f64 * t2518 * t50450 * t1354 + 0.12414802127193579148e5_f64 * t7799 * t14155 * t4884 - 0.14035736153892489771e2_f64 * t10485 * t16683 + t57327 + t57330 + t57332 + t57335 - t57337;
    t57338
}
