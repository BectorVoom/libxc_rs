//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1516/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1516(t119: f64, t1315: f64, t1831: f64, t20479: f64, t210: f64, t5240: f64, t554: f64, t559: f64, t56795: f64, t74311: f64, t74395: f64, t74401: f64, t74403: f64, t74405: f64, t74578: f64, t74584: f64, t74597: f64, t74618: f64, t79984: f64, t80175: f64) -> f64 {
    let t80375 = 7.0_f64 / 36.0_f64 * t74395 - 7.0_f64 / 192.0_f64 * t74401 + 7.0_f64 / 288.0_f64 * t74403 - 35.0_f64 / 96.0_f64 * t74405 + t80175 * t554 * t559 / 3072.0_f64 - 119.0_f64 / 288.0_f64 * t56795 - t5240 * t20479 / 192.0_f64 - t74311 * t1831 / 192.0_f64 - 7.0_f64 / 1152.0_f64 * t74578 + 7.0_f64 / 384.0_f64 * t74584 - 7.0_f64 / 96.0_f64 * t74597 + 7.0_f64 / 48.0_f64 * t74618 - t1315 * t210 * t119 * t79984 / 48.0_f64;
    t80375
}
