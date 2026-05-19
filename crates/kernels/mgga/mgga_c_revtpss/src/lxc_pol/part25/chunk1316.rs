//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1316/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1316<F: Float>(t2018: F, t807: F, t9714: F, t94494: F, t94498: F, t94501: F, t94503: F, t94505: F, t94509: F, t94511: F, t94514: F, t94517: F, t94520: F, t94523: F, t94526: F, t94527: F) -> F {
    let t94530 = t807 * t2018 * t9714;
    let t94532 = -F::cast_from(0.25724410870841842183e-2_f64) * t94494 + F::cast_from(0.81312004494856525162e-3_f64) * t94498 - F::cast_from(0.34299214494455789577e-3_f64) * t94501 + F::cast_from(0.60023625365297631762e-2_f64) * t94503 + F::cast_from(0.60023625365297631762e-2_f64) * t94505 + F::cast_from(0.15246000842785598468e-3_f64) * t94509 - F::cast_from(0.76230004213927992339e-4_f64) * t94511 - F::new(7.0) / F::new(16.0) * t94514 - t94517 / F::new(4.0) - F::new(35.0) / F::new(72.0) * t94520 - t94523 + t94526 - F::cast_from(0.18292914397043087774e-2_f64) * t94527 + F::cast_from(0.17149607247227894789e-3_f64) * t94530;
    t94532
}
