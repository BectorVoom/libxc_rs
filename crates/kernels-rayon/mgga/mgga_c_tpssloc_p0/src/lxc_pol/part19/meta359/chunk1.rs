//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1305/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1305(t10214: f64, t10378: f64, t1041: f64, t10463: f64, t10863: f64, t10879: f64, t248: f64, t2960: f64, t2979: f64, t3062: f64, t3098: f64, t3117: f64, t39097: f64, t41644: f64, t41693: f64, t41697: f64, t41701: f64, t41705: f64, t42303: f64, t42309: f64, t42322: f64, t42324: f64, t42334: f64, t973: f64, t974: f64, t977: f64) -> f64 {
    let t42337 = 2.0_f64 / 9.0_f64 * t2960 * t10378 + 7.0_f64 / 108.0_f64 * t973 * t10214 * t41693 + 5.0_f64 / 4608.0_f64 * t1041 * t248 * t3062 * t41701 + 19.0_f64 / 324.0_f64 * t42303 + t10863 * t3098 / 36.0_f64 + 35.0_f64 / 972.0_f64 * t973 * t974 * t42309 * t39097 - t973 * t977 * t41644 / 36.0_f64 + t973 * t2979 * t41705 / 54.0_f64 + t42322 / 1728.0_f64 + 5.0_f64 / 1728.0_f64 * t42324 + t3117 * t10463 / 1152.0_f64 + 5.0_f64 / 384.0_f64 * t1041 * t248 * t3062 * t41697 - t42334 * t10879 / 128.0_f64;
    t42337
}
