//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3104/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3104(t81509: f64, t81511: f64, t81514: f64, t81516: f64, t81518: f64, t81521: f64, t81523: f64, t81525: f64, t81527: f64, t81530: f64, t81533: f64, t81536: f64) -> f64 {
    let t81754 = -0.6618234375e1_f64 * t81509 + 0.794188125e1_f64 * t81511 - 0.17648625e1_f64 * t81514 - 0.52945875e1_f64 * t81516 - 0.52945875e1_f64 * t81518 + 0.2366859375e0_f64 * t81521 - 0.473371875e0_f64 * t81523 + 0.94674375e0_f64 * t81525 + 0.94674375e0_f64 * t81527 + 0.31558125e0_f64 * t81530 - 0.104195e0_f64 * t81533 - 0.104195e0_f64 * t81536;
    t81754
}
