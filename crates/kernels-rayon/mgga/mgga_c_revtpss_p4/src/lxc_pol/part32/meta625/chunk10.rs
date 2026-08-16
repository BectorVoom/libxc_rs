//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1988/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1988(t102499: f64, t102505: f64, t102508: f64, t102509: f64, t108566: f64, t108568: f64, t108570: f64, t108572: f64, t108574: f64, t108576: f64, t108578: f64, t98165: f64, t98174: f64) -> f64 {
    let t109808 = -0.18140473443734395377e0_f64 * t98165 - t102499 + 0.10841600599314203355e-2_f64 * t98174 - t102505 - 0.50820002809285328225e-4_f64 * t108566 - 0.85748036236139473944e-3_f64 * t108568 - 0.50820002809285328225e-4_f64 * t108570 - 0.10289764348336736873e0_f64 * t108572 + 0.34299214494455789578e-1_f64 * t108574 + 0.40015750243531754507e-2_f64 * t108576 + 0.17149607247227894789e-1_f64 * t108578 + t102508 - t102509;
    t109808
}
