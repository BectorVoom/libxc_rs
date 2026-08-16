//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1037/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1037(t120393: f64, t120416: f64, t115458: f64, t115463: f64, t115464: f64, t115467: f64, t117231: f64, t117232: f64, t117235: f64, t120388: f64, t120395: f64, t120397: f64, t120399: f64, t120401: f64, t120405: f64, t120408: f64, t120410: f64, t120413: f64, t120419: f64) -> f64 {
    let t124154 = 0.32298204875312312682e-2_f64 * t120393;
    let t124163 = 7.0_f64 / 576.0_f64 * t120416;
    let t124165 = 0.64596409750624625364e-2_f64 * t120388 + t115458 + t124154 + t120395 / 96.0_f64 - t120397 / 384.0_f64 + t120399 / 96.0_f64 + t120401 / 192.0_f64 + t117231 - 0.19378922925187387609e-1_f64 * t120405 - 0.32298204875312312682e-2_f64 * t120408 + 0.22608743412718618877e-1_f64 * t120410 + t120413 / 384.0_f64 - t124163 + 0.13565246047631171326e0_f64 * t120419 + t117232 + t115463 - t115464 + t117235 + t115467;
    t124165
}
