//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1106/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1106<F: Float>(t13160: F, t4595: F, t16287: F, t3441: F, t4649: F, t6: F, t127: F, t2024: F, t4615: F, t4599: F, t141: F, t22836: F, t135: F, t161: F, t2021: F, t29592: F, t3439: F, t3440: F, t38668: F, t38671: F, t38685: F, t38689: F, t48313: F, t48315: F, t48317: F, t48320: F, t48356: F, t48388: F, t629: F, t636: F, t6945: F, t9771: F) -> (F, F, F, F, F, F, F, F, F) {
    let t56193 = t13160 * t4595;
    let t56197 = t3441 * t16287;
    let t56203 = t4649 * t4649;
    let t56204 = t6 * t56203;
    let t56205 = t56204 * t127;
    let t56209 = t56204 * t2024;
    let t56213 = t4615 * t4595;
    let t56222 = t4599 * t4599;
    let t56224 = t22836 * t141 * t56222;
    let t56227 = 0.30426065214260652492e1 * t48313 + 0.15213032607130326245e0 * t48315 - 0.60852130428521304982e0 * t48317 - 0.60852130428521304982e0 * t48320 - 0.19559613352024705172e1 * t3439 * t9771 * t56193 + 0.21732903724471894636e0 * t3439 * t3440 * t56197 + 0.76628608687767569239e1 * t29592 - 0.43103592386869257697e0 * t38668 - 0.81498388966769604888e-2 * t636 * t161 * t56205 + 0.16299677793353920977e-1 * t2021 * t161 * t56209 - 3.0 / 2.0 * t6945 * t629 * t56213 - 0.43103592386869257697e0 * t38671 + 0.12170426085704260996e1 * t48356 + 0.86207184773738515393e1 * t38685 - 0.17241436954747703079e1 * t38689 + 0.5071010869043442082e-1 * t48388 + 0.22819548910695489368e1 * t135 * t56224;
    (t56193, t56197, t56203, t56205, t56209, t56213, t56222, t56224, t56227)
}
