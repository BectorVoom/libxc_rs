//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 178/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk178<F: Float>(t127: F, t332: F, t335: F, t367: F, t380: F, t392: F, t397: F, t417: F, t418: F, t500: F, t509: F, t515: F, t527: F, t532: F, t537: F, t542: F) -> (F,) {
    let t545 = -t332 - t335 * t500 / 48.0 + t127 * t509 / 96.0 - t367 * t515 / 96.0 + t380 - t392 - 0.21437009059034868486e-3 * t397 * t527 - t417 - 0.85748036236139473944e-3 * t418 * t532 + 0.42874018118069736972e-3 * t418 * t537 - 0.42874018118069736972e-3 * t418 * t542;
    (t545,)
}
