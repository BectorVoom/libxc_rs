//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1772/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1772(t45106: f64, t45107: f64, t89824: f64, t89832: f64, t90402: f64, t90405: f64, t90408: f64, t90411: f64, t90414: f64, t90417: f64, t90420: f64, t90423: f64, t90451: f64, t90453: f64) -> f64 {
    let t90701 = -0.104195e0_f64 * t90402 + 0.62517e0_f64 * t90405 - 0.125034e1_f64 * t90408 + 0.250068e1_f64 * t90411 + 0.104195e0_f64 * t90414 - 0.10805407407407407407e0_f64 * t90417 - 0.52945875e1_f64 * t90420 + 0.2366859375e0_f64 * t90423 - 0.15302962962962962963e1_f64 * t89832 + t45106 + t45107 + 0.6311625e0_f64 * t90451 - 0.6618234375e1_f64 * t90453 + 0.34431666666666666667e1_f64 * t89824;
    t90701
}
