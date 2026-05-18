//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1208/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1208<F: Float>(t94522: F, t94525: F, t94494: F, t94498: F, t94501: F, t94503: F, t94505: F, t94509: F, t94511: F, t94514: F, t94517: F, t94520: F, t94527: F, t94530: F) -> F {
    let t96341 = F::new(0.15117061203111996147e0) * t94522;
    let t96342 = F::new(0.80328230880474379779e-6) * t94525;
    let t96345 = -F::new(0.51448821741683684367e-2) * t94494 + F::new(0.16262400898971305032e-2) * t94498 - F::new(0.68598428988911579154e-3) * t94501 + F::new(0.12004725073059526352e-1) * t94503 + F::new(0.12004725073059526352e-1) * t94505 + F::new(0.30492001685571196935e-3) * t94509 - F::new(0.15246000842785598468e-3) * t94511 - F::new(7.0) / F::new(8.0) * t94514 - t94517 / F::new(2.0) - F::new(35.0) / F::new(36.0) * t94520 - t96341 + t96342 - F::new(0.3658582879408617555e-2) * t94527 + F::new(0.34299214494455789577e-3) * t94530;
    t96345
}
