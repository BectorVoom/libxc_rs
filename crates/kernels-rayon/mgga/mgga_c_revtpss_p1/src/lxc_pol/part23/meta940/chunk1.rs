//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3088/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3088(t141: f64, t3417: f64, t81186: f64, t81509: f64, t81511: f64, t81514: f64, t81516: f64, t81518: f64, t81521: f64, t81523: f64, t81525: f64, t81527: f64, t81530: f64, t81533: f64) -> (f64, f64) {
    let t81536 = t141 * t3417 * t81186;
    let t81538 = -0.3560484375e1_f64 * t81509 + 0.427258125e1_f64 * t81511 - 0.9494625e0_f64 * t81514 - 0.28483875e1_f64 * t81516 - 0.28483875e1_f64 * t81518 + 0.1151859375e0_f64 * t81521 - 0.230371875e0_f64 * t81523 + 0.46074375e0_f64 * t81525 + 0.46074375e0_f64 * t81527 + 0.15358125e0_f64 * t81530 - 0.82156666666666666668e-1_f64 * t81533 - 0.82156666666666666668e-1_f64 * t81536;
    (t81536, t81538)
}
