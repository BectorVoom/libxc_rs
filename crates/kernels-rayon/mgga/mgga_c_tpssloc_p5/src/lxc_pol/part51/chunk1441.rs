//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1441/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1441(t120410: f64, t120416: f64, t114013: f64, t114031: f64, t114035: f64, t114046: f64, t115461: f64, t115462: f64, t115465: f64, t120388: f64, t120393: f64, t120395: f64, t120397: f64, t120399: f64, t120401: f64, t120405: f64, t120408: f64, t120413: f64, t120419: f64) -> f64 {
    let t122432 = 0.11304371706359309439e-1_f64 * t120410;
    let t122434 = 7.0_f64 / 1152.0_f64 * t120416;
    let t122438 = 0.32298204875312312682e-2_f64 * t120388 + t114013 + 0.16149102437656156341e-2_f64 * t120393 + t120395 / 192.0_f64 - t120397 / 768.0_f64 + t120399 / 192.0_f64 + t120401 / 384.0_f64 + t115461 - 0.96894614625936938046e-2_f64 * t120405 - 0.16149102437656156341e-2_f64 * t120408 + t122432 + t120413 / 768.0_f64 - t122434 + 0.67826230238155856632e-1_f64 * t120419 + t115462 + 0.16149102437656156341e-2_f64 * t114031 - t114035 + t115465 + 0.26915170729426927235e-3_f64 * t114046;
    t122438
}
