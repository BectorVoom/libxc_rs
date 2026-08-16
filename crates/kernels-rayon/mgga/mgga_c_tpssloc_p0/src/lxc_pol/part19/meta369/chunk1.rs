//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1361/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1361(t10283: f64, t995: f64, t10931: f64, t135: f64, t973: f64, t1021: f64, t1046: f64, t10501: f64, t10998: f64, t248: f64, t2960: f64, t3048: f64, t350: f64, t42348: f64, t42759: f64, t43273: f64, t43277: f64, t43281: f64, t43285: f64, t43291: f64, t43292: f64, t43298: f64, t43301: f64, t43303: f64, t43307: f64) -> f64 {
    let t43310 = t10283 * t995;
    let t43313 = t973 * t135 * t10931;
    let t43315 = 5.0_f64 / 108.0_f64 * t3048 * t10501 - 2.0_f64 / 9.0_f64 * t2960 * t10998 + t43273 / 36.0_f64 + t43277 / 192.0_f64 - t43281 / 192.0_f64 + t43285 / 1152.0_f64 + t43291 * t248 * t1021 * t42348 * t43292 / 128.0_f64 - t43298 * t1046 / 72.0_f64 + t43301 / 384.0_f64 + 19.0_f64 / 216.0_f64 * t43303 - t43307 + 1309.0_f64 / 486.0_f64 * t42759 * t350 - 154.0_f64 / 243.0_f64 * t43310 - t43313 / 27.0_f64;
    t43315
}
