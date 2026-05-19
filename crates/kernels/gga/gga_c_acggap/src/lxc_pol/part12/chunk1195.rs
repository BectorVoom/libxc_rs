//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1195/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1195<F: Float>(t35403: F, t35407: F, t35410: F, t35418: F, t35425: F, t31212: F, t31222: F, t31224: F, t31231: F, t31237: F, t31239: F, t31241: F, t31245: F, t31247: F, t32739: F, t32740: F, t35415: F, t35422: F) -> F {
    let t37531 = F::cast_from(0.34299214494455789578e-2_f64) * t35403;
    let t37533 = t35407 / F::new(16.0);
    let t37534 = t35410 / F::new(48.0);
    let t37538 = F::cast_from(0.66040993808168719343e-1_f64) * t35418;
    let t37541 = F::cast_from(0.95275595817932748827e-2_f64) * t35425;
    let t37547 = t37531 + F::cast_from(0.56606566121287473723e-1_f64) * t31212 - t37533 - t37534 - F::cast_from(0.85748036236139473944e-3_f64) * t31222 - t35415 / F::new(16.0) - F::cast_from(0.90035438047946447644e-1_f64) * t31224 + t32739 + t37538 + F::cast_from(0.21437009059034868486e-2_f64) * t35422 + t32740 + F::cast_from(0.68598428988911579156e-2_f64) * t31231 + t37541 - F::cast_from(0.62896184579208304138e-3_f64) * t31237 - F::cast_from(0.62896184579208304138e-3_f64) * t31239 - F::cast_from(0.16772315887788881103e-2_f64) * t31241 + F::cast_from(0.62896184579208304138e-3_f64) * t31245 + F::cast_from(0.6431102717710460546e-2_f64) * t31247;
    t37547
}
