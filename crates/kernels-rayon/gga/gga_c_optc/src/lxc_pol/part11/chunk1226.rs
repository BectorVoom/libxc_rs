//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1226/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1226(t135: f64, t161: f64, t2021: f64, t29592: f64, t3439: f64, t3440: f64, t38668: f64, t38671: f64, t38685: f64, t38689: f64, t48313: f64, t48315: f64, t48317: f64, t48320: f64, t48356: f64, t48388: f64, t56193: f64, t56197: f64, t56205: f64, t56209: f64, t56213: f64, t56224: f64, t629: f64, t636: f64, t6945: f64, t9771: f64) -> f64 {
    let t56227 = 0.30426065214260652492e1_f64 * t48313 + 0.15213032607130326245e0_f64 * t48315 - 0.60852130428521304982e0_f64 * t48317 - 0.60852130428521304982e0_f64 * t48320 - 0.19559613352024705172e1_f64 * t3439 * t9771 * t56193 + 0.21732903724471894636e0_f64 * t3439 * t3440 * t56197 + 0.76628608687767569239e1_f64 * t29592 - 0.43103592386869257697e0_f64 * t38668 - 0.81498388966769604888e-2_f64 * t636 * t161 * t56205 + 0.16299677793353920977e-1_f64 * t2021 * t161 * t56209 - 3.0_f64 / 2.0_f64 * t6945 * t629 * t56213 - 0.43103592386869257697e0_f64 * t38671 + 0.12170426085704260996e1_f64 * t48356 + 0.86207184773738515393e1_f64 * t38685 - 0.17241436954747703079e1_f64 * t38689 + 0.5071010869043442082e-1_f64 * t48388 + 0.22819548910695489368e1_f64 * t135 * t56224;
    t56227
}
