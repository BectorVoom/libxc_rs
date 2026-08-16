//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1296/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1296(t14404: f64, t22379: f64, t51156: f64, t51168: f64, t53503: f64, t53509: f64, t53516: f64, t53795: f64, t54550: f64, t56483: f64, t56491: f64, t56495: f64, t56500: f64, t56505: f64, t56511: f64, t56514: f64, t6793: f64, t8793: f64) -> f64 {
    let t56518 = -t56483 / 48.0_f64 + t8793 * t54550 / 24.0_f64 + t22379 * t14404 / 24.0_f64 - t6793 * t56491 / 16.0_f64 - t56495 / 96.0_f64 + t56500 / 192.0_f64 + t56505 / 192.0_f64 - 35.0_f64 / 432.0_f64 * t51156 + 35.0_f64 / 216.0_f64 * t51168 - t53503 + t53509 - t56511 / 768.0_f64 + t53516 - t56514 / 24.0_f64 - t8793 * t53795 / 8.0_f64;
    t56518
}
