//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1360/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1360(t62711: f64, t63998: f64, t66423: f64, t66427: f64, t66429: f64, t66434: f64, t69989: f64, t69991: f64, t69993: f64, t69995: f64, t69997: f64, t69999: f64, t70001: f64) -> f64 {
    let t72077 = 5.0_f64 / 96.0_f64 * t69989 + 5.0_f64 / 192.0_f64 * t69991 + 7.0_f64 / 1152.0_f64 * t69993 + 7.0_f64 / 1152.0_f64 * t69995 - t69997 / 768.0_f64 - 7.0_f64 / 576.0_f64 * t69999 - 5.0_f64 / 32.0_f64 * t70001 - t62711 + t66423 + t66427 - t66429 - t66434 - t63998;
    t72077
}
