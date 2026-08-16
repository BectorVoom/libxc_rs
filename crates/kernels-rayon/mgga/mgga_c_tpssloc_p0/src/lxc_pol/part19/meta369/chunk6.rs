//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1366/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1366(t43398: f64, t61: f64, t10309: f64, t1041: f64, t10457: f64, t248: f64, t10444: f64, t354: f64, t364: f64, t372: f64, t1021: f64, t10364: f64, t10408: f64, t10413: f64, t1046: f64, t10482: f64, t10962: f64, t10965: f64, t10972: f64, t2771: f64, t2960: f64, t3041: f64, t3057: f64, t3064: f64, t3117: f64, t3123: f64, t41667: f64, t41715: f64, t42348: f64, t43374: f64, t43377: f64, t43382: f64, t43385: f64, t973: f64, t977: f64) -> f64 {
    let t43399 = t61 * t43398;
    let t43406 = t1041 * t248 * t10457 * t10309;
    let t43410 = t354 * t364 * t10444 * t372;
    let t43415 = -5.0_f64 / 2304.0_f64 * t10413 * t10408 * t3041 * t2771 - 4.0_f64 / 27.0_f64 * t2960 * t10364 - t43374 / 36.0_f64 + t43377 / 54.0_f64 + t973 * t977 * t41715 / 8.0_f64 + t43382 / 2592.0_f64 - 3.0_f64 / 256.0_f64 * t43385 * t248 * t1021 * t42348 * t10482 + t10965 * t3057 / 768.0_f64 + 5.0_f64 / 2304.0_f64 * t10965 * t3064 + 5.0_f64 / 1296.0_f64 * t3117 * t10972 + 55.0_f64 / 15552.0_f64 * t1041 * t248 * t43399 * t41667 - 5.0_f64 / 864.0_f64 * t43406 - 209.0_f64 / 972.0_f64 * t43410 * t1046 + t10962 * t3123 / 512.0_f64;
    t43415
}
