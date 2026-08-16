//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1762/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1762(t44039: f64, t44040: f64, t89824: f64, t89832: f64, t90402: f64, t90405: f64, t90408: f64, t90411: f64, t90414: f64, t90417: f64, t90420: f64, t90423: f64, t90451: f64, t90453: f64) -> f64 {
    let t90542 = -0.82156666666666666668e-1_f64 * t90402 + 0.49293999999999999999e0_f64 * t90405 - 0.98587999999999999998e0_f64 * t90408 + 0.197176e1_f64 * t90411 + 0.82156666666666666667e-1_f64 * t90414 - 0.85199506172839506175e-1_f64 * t90417 - 0.28483875e1_f64 * t90420 + 0.1151859375e0_f64 * t90423 - 0.88582716049382716048e0_f64 * t89832 + t44039 + t44040 + 0.3071625e0_f64 * t90451 - 0.3560484375e1_f64 * t90453 + 0.19931111111111111111e1_f64 * t89824;
    t90542
}
