//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1132/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1132(t1089: f64, t12421: f64, t1298: f64, t1345: f64, t1427: f64, t146: f64, t1487: f64, t15337: f64, t15814: f64, t20238: f64, t20263: f64, t20268: f64, t20273: f64, t20275: f64, t368: f64, t4099: f64, t418: f64, t4255: f64, t4256: f64, t4875: f64, t495: f64, t506: f64) -> f64 {
    let t20278 = -0.17149607247227894789e-2_f64 * t20238 - t15814 * t146 * t1427 * t1298 - t4255 * t4256 * t1345 * t1298 / 4.0_f64 + 0.24009450146119052704e-1_f64 * t15337 - 0.34299214494455789578e-2_f64 * t418 * t1089 * t368 * t4099 * t506 - 0.68598428988911579156e-2_f64 * t418 * t1089 * t368 * t1298 * t1487 - 0.34299214494455789578e-2_f64 * t418 * t1089 * t368 * t495 * t4875 + 0.17149607247227894789e-2_f64 * t20263 + 0.17149607247227894789e-2_f64 * t20268 + 0.85748036236139473944e-3_f64 * t20273 - 0.85748036236139473944e-2_f64 * t20275 - 0.24009450146119052705e-1_f64 * t12421;
    t20278
}
