//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1349/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1349<F: Float>(t2634: F, t6212: F, t20589: F, t6211: F, t2612: F, t20237: F, t6118: F, t7970: F, t24581: F, t2559: F, t1543: F, t20661: F, t20667: F, t20688: F, t20696: F, t20710: F, t20729: F, t20731: F, t22997: F, t24877: F, t2591: F, t2598: F, t2719: F, t360: F, t5137: F, t551: F, t552: F, t6449: F, t938: F) -> (F,) {
    let t25499 = t6212 * t2634;
    let t25501 = t20589 * t6211 * t25499;
    let t25503 = t6212 * t2612;
    let t25505 = t20237 * t6211 * t25503;
    let t25518 = t6118 * t7970;
    let t25520 = t24581 * t2559;
    let t25521 = 0.64025200389650807209e0 * t25520;
    let t25527 = 0.10401866088065122276e1 * t22997 * t551 * t552 * t938 * t5137 + 0.38087975358139160777e-1 * t25501 + 0.57131963037208741166e-1 * t25505 + 0.86743646395112941038e-3 * t20661 + 0.38087975358139160777e-1 * t20667 + 0.12805040077930161442e1 * t20688 - 0.10401866088065122276e1 * t20696 + 0.69345773920434148506e0 * t20710 + 0.20803732176130244552e1 * t20729 + 0.26004665220162805689e0 * t2598 * t360 * t24877 * t2591 + 0.19207560116895242163e0 * t20731 - 0.38415120233790484326e0 * t25518 - t25521 - 0.15602799132097683414e1 * t6449 * t551 * t552 * t2719 * t1543;
    (t25527,)
}
