//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1256/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1256(t2018: f64, t807: f64, t9714: f64, t94494: f64, t94498: f64, t94501: f64, t94503: f64, t94505: f64, t94509: f64, t94511: f64, t94514: f64, t94517: f64, t94520: f64, t94523: f64, t94526: f64, t94527: f64) -> f64 {
    let t94530 = t807 * t2018 * t9714;
    let t94532 = -0.25724410870841842183e-2_f64 * t94494 + 0.81312004494856525162e-3_f64 * t94498 - 0.34299214494455789577e-3_f64 * t94501 + 0.60023625365297631762e-2_f64 * t94503 + 0.60023625365297631762e-2_f64 * t94505 + 0.15246000842785598468e-3_f64 * t94509 - 0.76230004213927992339e-4_f64 * t94511 - 7.0_f64 / 16.0_f64 * t94514 - t94517 / 4.0_f64 - 35.0_f64 / 72.0_f64 * t94520 - t94523 + t94526 - 0.18292914397043087774e-2_f64 * t94527 + 0.17149607247227894789e-3_f64 * t94530;
    t94532
}
