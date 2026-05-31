//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1196/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1196<F: Float>(t108566: F, t108570: F, t108576: F, t114541: F, t114543: F, t114545: F, t114547: F, t114549: F, t114551: F, t114553: F, t96321: F, t96322: F, t98174: F, t98200: F) -> F {
    let t115040 = F::cast_from(0.16262400898971305032e-2_f64) * t98174 - F::cast_from(0.15246000842785598468e-3_f64) * t108566 - F::cast_from(0.15246000842785598468e-3_f64) * t108570 + F::cast_from(0.12004725073059526352e-1_f64) * t108576 - t114541 / F::cast_from(24.0_f64) - t114543 / F::cast_from(2.0_f64) - t96321 - F::cast_from(0.25724410870841842183e-2_f64) * t114545 - F::cast_from(0.51448821741683684367e-1_f64) * t114547 + F::cast_from(0.10289764348336736873e-1_f64) * t114549 - F::cast_from(0.20579528696673473747e-1_f64) * t114551 + F::cast_from(0.51448821741683684367e-2_f64) * t114553 + F::cast_from(0.12196800674228478774e-3_f64) * t98200 + t96322;
    t115040
}
