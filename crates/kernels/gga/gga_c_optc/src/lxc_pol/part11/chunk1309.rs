//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1309/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1309<F: Float>(t10419: F, t10478: F, t10485: F, t1354: F, t14098: F, t14155: F, t16677: F, t16680: F, t16683: F, t16771: F, t2493: F, t2518: F, t4869: F, t4884: F, t4888: F, t4919: F, t50450: F, t57211: F, t57246: F, t57260: F, t57327: F, t57330: F, t57332: F, t57335: F, t57337: F, t7753: F, t7759: F, t7799: F) -> F {
    let t57338 = F::new(0.61523382126046769581e4) * t7753 * t14098 * t4919 - F::new(24.0) * t10419 * t16677 + F::new(0.38597619813444837568e3) * t10478 * t16680 + t57246 - F::new(0.19751789702565206229e-1) * t57211 - t57260 + F::new(36.0) * t2518 * t4869 * t4884 - F::new(0.11579285944033451271e4) * t7759 * t4888 * t4884 - F::new(8.0) * t2493 * t16771 * t1354 + F::new(0.1286587327114827919e3) * t2518 * t50450 * t1354 + F::new(0.12414802127193579148e5) * t7799 * t14155 * t4884 - F::new(0.14035736153892489771e2) * t10485 * t16683 + t57327 + t57330 + t57332 + t57335 - t57337;
    t57338
}
