//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3114/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3114(t81509: f64, t81511: f64, t81514: f64, t81516: f64, t81518: f64, t81521: f64, t81523: f64, t81525: f64, t81527: f64, t81530: f64, t81533: f64, t81536: f64) -> f64 {
    let t81983 = -0.485484375e1_f64 * t81509 + 0.58258125e1_f64 * t81511 - 0.1294625e1_f64 * t81514 - 0.3883875e1_f64 * t81516 - 0.3883875e1_f64 * t81518 + 0.6189328125e-1_f64 * t81521 - 0.1237865625e0_f64 * t81523 + 0.247573125e0_f64 * t81525 + 0.247573125e0_f64 * t81527 + 0.82524375e-1_f64 * t81530 - 0.82785e-1_f64 * t81533 - 0.82785e-1_f64 * t81536;
    t81983
}
