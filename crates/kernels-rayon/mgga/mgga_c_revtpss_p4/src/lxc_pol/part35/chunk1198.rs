//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1198/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1198(t108604: f64, t108608: f64, t108623: f64, t108625: f64, t108627: f64, t108629: f64, t114573: f64, t114575: f64, t114577: f64, t114584: f64, t114586: f64, t96358: f64, t96359: f64, t98285: f64) -> f64 {
    let t115065 = -0.17149607247227894789e-3_f64 * t108604 - 0.6098400337114239387e-3_f64 * t108608 - 0.51448821741683684367e-2_f64 * t114573 + 0.51448821741683684367e-1_f64 * t114575 - 0.85748036236139473944e-3_f64 * t114577 - t96358 - t96359 - 0.2168591159877823526e-3_f64 * t98285 + 0.85748036236139473944e-4_f64 * t108623 + 0.30492001685571196935e-2_f64 * t108625 - 0.24009450146119052704e0_f64 * t108627 + 0.48018900292238105409e-1_f64 * t108629 - 0.34299214494455789578e-2_f64 * t114584 - 0.10289764348336736873e0_f64 * t114586;
    t115065
}
