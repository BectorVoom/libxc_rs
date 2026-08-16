//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1209/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1209(t94568: f64, t94570: f64, t94534: f64, t94537: f64, t94540: f64, t94542: f64, t94546: f64, t94548: f64, t94552: f64, t94554: f64, t94557: f64, t94559: f64, t94561: f64, t94565: f64) -> f64 {
    let t96358 = 0.45178982497454656792e-6_f64 * t94568;
    let t96359 = 0.28900264064772933812e-2_f64 * t94570;
    let t96360 = -0.17149607247227894789e-2_f64 * t94534 + 0.30492001685571196935e-4_f64 * t94537 - 0.2168591159877823526e-3_f64 * t94540 - 0.6098400337114239387e-3_f64 * t94542 - 0.27210710165601593065e0_f64 * t94546 + 0.48018900292238105409e-1_f64 * t94548 - 0.17149607247227894789e-3_f64 * t94552 - 0.91464571985215438874e-3_f64 * t94554 + 0.85748036236139473944e-4_f64 * t94557 - 0.24009450146119052704e0_f64 * t94559 + 0.30492001685571196935e-2_f64 * t94561 - 0.54214778996945588151e-4_f64 * t94565 - t96358 - t96359;
    t96360
}
