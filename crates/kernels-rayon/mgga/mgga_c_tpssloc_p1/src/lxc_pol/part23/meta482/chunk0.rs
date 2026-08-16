//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1451/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1451(t6218: f64, t11668: f64, t11779: f64, t1214: f64, t1227: f64, t15615: f64, t1735: f64, t1748: f64, t19033: f64, t21745: f64, t21749: f64, t22197: f64, t22208: f64, t248: f64, t3506: f64, t3508: f64, t3577: f64, t3578: f64, t4582: f64, t47: f64, t471: f64, t479: f64, t488: f64, t5005: f64, t6207: f64, t65600: f64, t65605: f64, t72255: f64, t72352: f64, t72366: f64, t77606: f64, t77957: f64, t8025: f64) -> (f64, f64) {
    let t78757 = t6218 * t6218;
    let t78775 = -t3577 * t3578 * t1735 * t21749 / 192.0_f64 + 5.0_f64 / 1152.0_f64 * t3577 * t11668 * t1735 * t21745 + 5.0_f64 / 1152.0_f64 * t5005 * t22197 - t1227 * t4582 * t15615 * t77606 / 128.0_f64 - 5.0_f64 / 1296.0_f64 * t5005 * t22208 - 5.0_f64 / 432.0_f64 * t1227 * t248 * t11779 * t77957 - t72255 * t1748 / 1152.0_f64 + t3506 * t248 * t1214 * t78757 * t3508 / 512.0_f64 - 11.0_f64 / 81.0_f64 * t72352 + t65600 / 216.0_f64 - t65605 / 1152.0_f64 + 5225.0_f64 / 7776.0_f64 * t471 * t479 / t47 / t8025 * t488 - 19.0_f64 / 432.0_f64 * t19033 * t6207 + t72366 / 384.0_f64;
    (t78757, t78775)
}
