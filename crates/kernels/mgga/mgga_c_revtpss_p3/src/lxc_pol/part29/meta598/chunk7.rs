//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2034/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2034<F: Float>(t100944: F, t100947: F, t100953: F, t100958: F, t100969: F, t100978: F, t101029: F, t101032: F, t101086: F, t102851: F, t102858: F, t1711: F, t1940: F, t26425: F, t26581: F, t27793: F, t27800: F, t28291: F, t3351: F, t7432: F, t8020: F, t95511: F) -> F {
    let t103750 = F::new(6.0) * t26425 * t100978 + F::new(2.0) * t102851 * t27800 - F::new(3.0) * t26425 * t100944 + t1940 * t26581 * t1711 / F::new(2.0) - F::new(3.0) * t28291 * t100958 + F::new(3.0) * t26425 * t101086 + t1940 * t8020 * t3351 / F::new(2.0) - F::new(3.0) * t95511 * t27793 + F::new(6.0) * t28291 * t101029 + F::new(6.0) * t28291 * t101032 - F::new(3.0) / F::new(2.0) * t26425 * t100947 - t1940 * t7432 * t100969 / F::new(2.0) - t102858 - F::new(6.0) * t28291 * t100953;
    t103750
}
