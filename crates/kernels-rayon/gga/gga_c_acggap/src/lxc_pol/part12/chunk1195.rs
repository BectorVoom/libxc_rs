//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1195/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1195(t35403: f64, t35407: f64, t35410: f64, t35418: f64, t35425: f64, t31212: f64, t31222: f64, t31224: f64, t31231: f64, t31237: f64, t31239: f64, t31241: f64, t31245: f64, t31247: f64, t32739: f64, t32740: f64, t35415: f64, t35422: f64) -> f64 {
    let t37531 = 0.34299214494455789578e-2_f64 * t35403;
    let t37533 = t35407 / 16.0_f64;
    let t37534 = t35410 / 48.0_f64;
    let t37538 = 0.66040993808168719343e-1_f64 * t35418;
    let t37541 = 0.95275595817932748827e-2_f64 * t35425;
    let t37547 = t37531 + 0.56606566121287473723e-1_f64 * t31212 - t37533 - t37534 - 0.85748036236139473944e-3_f64 * t31222 - t35415 / 16.0_f64 - 0.90035438047946447644e-1_f64 * t31224 + t32739 + t37538 + 0.21437009059034868486e-2_f64 * t35422 + t32740 + 0.68598428988911579156e-2_f64 * t31231 + t37541 - 0.62896184579208304138e-3_f64 * t31237 - 0.62896184579208304138e-3_f64 * t31239 - 0.16772315887788881103e-2_f64 * t31241 + 0.62896184579208304138e-3_f64 * t31245 + 0.6431102717710460546e-2_f64 * t31247;
    t37547
}
