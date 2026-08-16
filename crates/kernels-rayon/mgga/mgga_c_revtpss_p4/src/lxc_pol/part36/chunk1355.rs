//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1355/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1355(t104963: f64, t112433: f64, t112435: f64, t112437: f64, t112452: f64, t112461: f64, t112465: f64, t112468: f64, t24236: f64, t24636: f64, t24794: f64, t24798: f64, t26867: f64, t29047: f64, t29054: f64, t29089: f64, t6653: f64, t7613: f64, t97272: f64) -> f64 {
    let t116234 = t29047 * t29054 * t24236 / 72.0_f64 + t112433 / 54.0_f64 - t112435 / 288.0_f64 - t112437 / 144.0_f64 - 0.17149607247227894789e-2_f64 * t26867 * t24798 - 0.85748036236139473944e-3_f64 * t26867 * t24794 - 0.85748036236139473944e-3_f64 * t112452 - t29089 * t6653 / 27.0_f64 + t104963 / 54.0_f64 + t97272 + 0.85748036236139473944e-3_f64 * t112461 - 0.91464571985215438873e-2_f64 * t112465 + 0.11433071498151929859e-2_f64 * t112468 - 0.42874018118069736972e-3_f64 * t7613 * t24636;
    t116234
}
