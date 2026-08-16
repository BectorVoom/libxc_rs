//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 298/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk298(t1131: f64, t336: f64, t368: f64, t1044: f64, t1080: f64, t1086: f64, t1092: f64, t1098: f64, t1104: f64, t1109: f64, t1114: f64, t1117: f64, t1121: f64, t127: f64, t335: f64, t367: f64, t418: f64) -> (f64, f64) {
    let t1133 = t336 * t368 * t1131;
    let t1136 = -7.0_f64 / 144.0_f64 * t1044 + t127 * t1080 / 96.0_f64 - 0.17149607247227894789e-2_f64 * t418 * t1086 - 0.34299214494455789578e-2_f64 * t418 * t1092 + 0.34299214494455789578e-2_f64 * t418 * t1098 + 0.17149607247227894789e-2_f64 * t1104 - 0.85748036236139473944e-3_f64 * t1109 + 0.85748036236139473944e-3_f64 * t1114 - t335 * t1117 / 48.0_f64 - t367 * t1121 / 48.0_f64 - t367 * t1133 / 96.0_f64;
    (t1133, t1136)
}
