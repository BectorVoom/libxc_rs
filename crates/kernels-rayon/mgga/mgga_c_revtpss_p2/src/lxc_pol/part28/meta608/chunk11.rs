//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2118/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2118(t94484: f64, t94485: f64, t94498: f64, t94501: f64, t94503: f64, t94505: f64, t94509: f64, t94511: f64, t98236: f64, t98239: f64, t98244: f64, t98245: f64, t98253: f64) -> f64 {
    let t98255 = -t98236 + t98239 + t94484 + 7.0_f64 / 144.0_f64 * t94485 + t98244 + 0.34299214494455789578e-2_f64 * t98245 + 0.54208002996571016774e-3_f64 * t94498 - 0.11433071498151929859e-3_f64 * t94501 + 0.20007875121765877254e-2_f64 * t94503 + 0.20007875121765877254e-2_f64 * t94505 + 0.50820002809285328226e-4_f64 * t94509 - 0.25410001404642664113e-4_f64 * t94511 - t98253 / 48.0_f64;
    t98255
}
