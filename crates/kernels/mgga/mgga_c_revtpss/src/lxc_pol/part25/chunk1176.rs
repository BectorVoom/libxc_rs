//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1176/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1176<F: Float>(t94525: F, t26014: F, t2689: F, t2018: F, t807: F, t9714: F, t94494: F, t94498: F, t94501: F, t94503: F, t94505: F, t94509: F, t94511: F, t94514: F, t94517: F, t94520: F, t94523: F) -> (F,) {
    let t94526 = 0.4016411544023718989e-6 * t94525;
    let t94527 = t2689 * t26014;
    let t94530 = t807 * t2018 * t9714;
    let t94532 = -0.25724410870841842183e-2 * t94494 + 0.81312004494856525162e-3 * t94498 - 0.34299214494455789577e-3 * t94501 + 0.60023625365297631762e-2 * t94503 + 0.60023625365297631762e-2 * t94505 + 0.15246000842785598468e-3 * t94509 - 0.76230004213927992339e-4 * t94511 - 7.0 / 16.0 * t94514 - t94517 / 4.0 - 35.0 / 72.0 * t94520 - t94523 + t94526 - 0.18292914397043087774e-2 * t94527 + 0.17149607247227894789e-3 * t94530;
    (t94532,)
}
