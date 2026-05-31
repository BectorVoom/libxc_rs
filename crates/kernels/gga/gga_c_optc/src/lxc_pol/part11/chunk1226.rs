//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1226/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1226<F: Float>(t135: F, t161: F, t2021: F, t29592: F, t3439: F, t3440: F, t38668: F, t38671: F, t38685: F, t38689: F, t48313: F, t48315: F, t48317: F, t48320: F, t48356: F, t48388: F, t56193: F, t56197: F, t56205: F, t56209: F, t56213: F, t56224: F, t629: F, t636: F, t6945: F, t9771: F) -> F {
    let t56227 = F::cast_from(0.30426065214260652492e1_f64) * t48313 + F::cast_from(0.15213032607130326245e0_f64) * t48315 - F::cast_from(0.60852130428521304982e0_f64) * t48317 - F::cast_from(0.60852130428521304982e0_f64) * t48320 - F::cast_from(0.19559613352024705172e1_f64) * t3439 * t9771 * t56193 + F::cast_from(0.21732903724471894636e0_f64) * t3439 * t3440 * t56197 + F::cast_from(0.76628608687767569239e1_f64) * t29592 - F::cast_from(0.43103592386869257697e0_f64) * t38668 - F::cast_from(0.81498388966769604888e-2_f64) * t636 * t161 * t56205 + F::cast_from(0.16299677793353920977e-1_f64) * t2021 * t161 * t56209 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t6945 * t629 * t56213 - F::cast_from(0.43103592386869257697e0_f64) * t38671 + F::cast_from(0.12170426085704260996e1_f64) * t48356 + F::cast_from(0.86207184773738515393e1_f64) * t38685 - F::cast_from(0.17241436954747703079e1_f64) * t38689 + F::cast_from(0.5071010869043442082e-1_f64) * t48388 + F::cast_from(0.22819548910695489368e1_f64) * t135 * t56224;
    t56227
}
