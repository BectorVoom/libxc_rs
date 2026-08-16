//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1208/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1208(t94522: f64, t94525: f64, t94494: f64, t94498: f64, t94501: f64, t94503: f64, t94505: f64, t94509: f64, t94511: f64, t94514: f64, t94517: f64, t94520: f64, t94527: f64, t94530: f64) -> f64 {
    let t96341 = 0.15117061203111996147e0_f64 * t94522;
    let t96342 = 0.80328230880474379779e-6_f64 * t94525;
    let t96345 = -0.51448821741683684367e-2_f64 * t94494 + 0.16262400898971305032e-2_f64 * t94498 - 0.68598428988911579154e-3_f64 * t94501 + 0.12004725073059526352e-1_f64 * t94503 + 0.12004725073059526352e-1_f64 * t94505 + 0.30492001685571196935e-3_f64 * t94509 - 0.15246000842785598468e-3_f64 * t94511 - 7.0_f64 / 8.0_f64 * t94514 - t94517 / 2.0_f64 - 35.0_f64 / 36.0_f64 * t94520 - t96341 + t96342 - 0.3658582879408617555e-2_f64 * t94527 + 0.34299214494455789577e-3_f64 * t94530;
    t96345
}
