//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1387/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1387(t1021: f64, t10403: f64, t1041: f64, t1044: f64, t14211: f64, t14508: f64, t14511: f64, t1616: f64, t21138: f64, t21487: f64, t21503: f64, t21597: f64, t21603: f64, t21609: f64, t248: f64, t3070: f64, t3071: f64, t3130: f64, t3131: f64, t3151: f64, t42444: f64, t4582: f64, t4641: f64, t4644: f64, t5685: f64, t5873: f64, t62137: f64, t62148: f64, t62177: f64, t62183: f64, t70391: f64, t70497: f64, t75836: f64, t75847: f64, t76576: f64, t76616: f64, t76722: f64, t973: f64, t974: f64, t977: f64) -> f64 {
    let t77587 = t4644 * t21609 / 192.0_f64 + t3130 * t4582 * t70391 * t14211 / 384.0_f64 + t14508 * t21487 / 128.0_f64 - t14511 * t21503 / 256.0_f64 + t62137 / 1728.0_f64 - t62148 / 1152.0_f64 - t973 * t974 * t3151 * t75847 / 48.0_f64 - t62177 / 2304.0_f64 + t62183 / 2304.0_f64 + t4641 * t21597 / 768.0_f64 + t4644 * t21603 / 1152.0_f64 + t1041 * t248 * t1044 * t76576 / 4608.0_f64 + t70497 / 36.0_f64 + t973 * t977 * t76616 / 8.0_f64 + t3130 * t248 * t1021 * t76722 * t3131 / 512.0_f64 + t3070 * t3071 * t21138 * t1616 / 192.0_f64 - t973 * t974 * t42444 * t75836 / 12.0_f64 + t10403 * t3071 * t5873 * t5685 / 384.0_f64;
    t77587
}
