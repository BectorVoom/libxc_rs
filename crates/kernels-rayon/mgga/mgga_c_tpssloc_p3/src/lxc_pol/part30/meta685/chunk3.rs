//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2164/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2164(t22705: f64, t22852: f64, t236: f64, t550: f64, t6414: f64, t80784: f64, t80792: f64, t80794: f64, t80826: f64, t80837: f64, t80848: f64, t91282: f64, t91284: f64, t91287: f64, t91290: f64, t91301: f64, t97352: f64, t97354: f64, t97359: f64, t97361: f64, t97363: f64, t97367: f64) -> f64 {
    let t97372 = t22852 * t22705 * t236 * t6414 * t550;
    let t97376 = -t97352 / 384.0_f64 + 5.0_f64 / 384.0_f64 * t97354 + 0.16821981705891829522e-4_f64 * t80784 - 0.52708876011794399171e-3_f64 * t80792 + 119.0_f64 / 6912.0_f64 * t80794 - t80826 + 5.0_f64 / 384.0_f64 * t97359 + 5.0_f64 / 192.0_f64 * t97361 - 7.0_f64 / 2304.0_f64 * t97363 - 0.6728792682356731809e-4_f64 * t97367 + 0.33643963411783659045e-4_f64 * t97372 + 0.10093189023535097713e-3_f64 * t80837 - t80848 + t91282 + t91284 + t91287 - 0.16956557559538964159e-1_f64 * t91290 - t91301;
    t97376
}
