//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1465/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1465(t1227: f64, t1230: f64, t15569: f64, t1653: f64, t19026: f64, t19051: f64, t22214: f64, t22218: f64, t22288: f64, t22307: f64, t248: f64, t3578: f64, t44828: f64, t45197: f64, t5005: f64, t6207: f64, t6211: f64, t6221: f64, t6227: f64, t65541: f64, t65703: f64, t72470: f64, t72495: f64, t72501: f64, t77961: f64, t77969: f64) -> f64 {
    let t79056 = -t19051 * t6207 / 768.0_f64 - t5005 * t22214 / 1152.0_f64 - t19051 * t6211 / 384.0_f64 - t5005 * t22218 / 192.0_f64 + t72470 / 192.0_f64 + t15569 * t22288 / 36.0_f64 - t72495 / 288.0_f64 + 19.0_f64 / 288.0_f64 * t19026 * t6221 - t72501 / 288.0_f64 - t1227 * t248 * t1230 * t77969 / 768.0_f64 - t65703 * t6227 / 24.0_f64 + 55.0_f64 / 15552.0_f64 * t1227 * t248 * t44828 * t77961 + 19.0_f64 / 144.0_f64 * t65541 * t6227 - t45197 * t3578 * t22307 * t1653 / 192.0_f64;
    t79056
}
