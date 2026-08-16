//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1310/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1310(t10970: f64, t820: f64, t10418: f64, t10422: f64, t3070: f64, t1021: f64, t1023: f64, t10305: f64, t10316: f64, t10321: f64, t10403: f64, t10408: f64, t1041: f64, t10426: f64, t10483: f64, t10883: f64, t10886: f64, t248: f64, t2771: f64, t3041: f64, t3071: f64, t3131: f64, t3132: f64, t360: f64, t42347: f64, t42348: f64, t42354: f64, t42358: f64, t42369: f64, t42372: f64, t42374: f64, t42380: f64, t42388: f64, t4582: f64, t4583: f64, t884: f64) -> f64 {
    let t42397 = t820 * t10970;
    let t42403 = t3070 * t10422 * t10418;
    let t42409 = 7.0_f64 / 1536.0_f64 * t42347 * t248 * t1021 * t42348 * t3131 + t42354 * t10886 / 768.0_f64 - t42358 * t248 * t1021 * t42348 * t360 / 3072.0_f64 + t10883 * t4582 * t10426 * t3041 / 512.0_f64 - t42369 / 288.0_f64 + 5.0_f64 / 1728.0_f64 * t42372 - t1041 * t4582 * t4583 * t42374 / 576.0_f64 + t42380 / 288.0_f64 + t3070 * t3071 * t10316 * t1023 / 192.0_f64 + t42388 * t3071 * t10483 * t884 / 192.0_f64 + 5.0_f64 / 1152.0_f64 * t10403 * t10408 * t3132 * t2771 + 5.0_f64 / 1296.0_f64 * t3070 * t42397 * t10305 * t1023 - t42403 / 288.0_f64 + t3070 * t3071 * t10321 * t1023 / 1152.0_f64;
    t42409
}
