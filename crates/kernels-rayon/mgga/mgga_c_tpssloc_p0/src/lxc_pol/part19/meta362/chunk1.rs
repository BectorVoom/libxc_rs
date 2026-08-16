//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1315/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1315(t10390: f64, t10394: f64, t10398: f64, t1041: f64, t10428: f64, t10433: f64, t10884: f64, t10891: f64, t10904: f64, t10915: f64, t10919: f64, t10932: f64, t14187: f64, t2960: f64, t3048: f64, t3071: f64, t3073: f64, t42460: f64, t42468: f64, t42478: f64, t42481: f64, t42483: f64, t42490: f64, t42496: f64, t4582: f64, t884: f64) -> f64 {
    let t42499 = 2.0_f64 / 27.0_f64 * t42460 + 8.0_f64 / 27.0_f64 * t2960 * t10932 - t10904 * t10428 / 24.0_f64 + t10891 * t10433 / 48.0_f64 + 5.0_f64 / 864.0_f64 * t1041 * t4582 * t14187 * t42468 + t3048 * t10915 / 36.0_f64 - 5.0_f64 / 216.0_f64 * t3048 * t10919 - t42478 / 576.0_f64 + t42481 / 576.0_f64 + t42483 * t3071 * t10884 * t884 / 1152.0_f64 + 5.0_f64 / 1728.0_f64 * t42490 + t10390 * t10394 / 384.0_f64 + t10390 * t10398 / 384.0_f64 - t42496 * t3073 / 36.0_f64;
    t42499
}
