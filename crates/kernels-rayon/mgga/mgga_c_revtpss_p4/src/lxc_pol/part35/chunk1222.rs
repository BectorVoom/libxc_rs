//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1222/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1222(t106024: f64, t106030: f64, t106033: f64, t106037: f64, t106040: f64, t106042: f64, t106048: f64, t106050: f64, t95671: f64, t98976: f64, t98979: f64, t99002: f64, t99009: f64, t99013: f64) -> f64 {
    let t115687 = -0.24009450146119052704e0_f64 * t106024 - 0.2168591159877823526e-3_f64 * t98976 + 0.30492001685571196935e-4_f64 * t98979 + 0.16262400898971305032e-2_f64 * t99002 - t95671 - 0.27210710165601593065e0_f64 * t99009 - 0.17149607247227894789e-3_f64 * t106030 + 0.85748036236139473944e-4_f64 * t106033 + 0.65049603595885220128e-2_f64 * t99013 - 0.6098400337114239387e-3_f64 * t106037 + 0.85748036236139473944e-4_f64 * t106040 + 0.12004725073059526352e-1_f64 * t106042 - 0.15246000842785598468e-3_f64 * t106048 + 0.30492001685571196935e-3_f64 * t106050;
    t115687
}
