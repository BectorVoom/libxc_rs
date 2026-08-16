//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 1027/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk1027(t115461: f64, t115462: f64, t115465: f64, t120410: f64, t124154: f64, t124163: f64, t127278: f64, t127283: f64, t127285: f64, t127289: f64, t127293: f64, t127296: f64, t127299: f64) -> f64 {
    let t128625 = t127278 / 768.0_f64 + t124154 + t115461 + t127283 / 384.0_f64 - t127285 / 384.0_f64 - t127289 / 768.0_f64 - t127293 / 768.0_f64 + 0.22608743412718618878e-1_f64 * t120410 - t124163 + t115462 - 0.16149102437656156341e-2_f64 * t127296 + t115465 + 0.32298204875312312682e-2_f64 * t127299;
    t128625
}
