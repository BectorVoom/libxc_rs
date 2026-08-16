//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1334/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1334(t14535: f64, t3113: f64, t54199: f64, t57028: f64, t57031: f64, t57036: f64, t57038: f64, t57040: f64, t57042: f64, t57044: f64, t57046: f64, t57048: f64, t57050: f64, t57052: f64) -> f64 {
    let t57054 = t3113 * t14535;
    let t57056 = t57028 / 48.0_f64 - t57031 / 48.0_f64 + t57036 / 48.0_f64 - t57038 / 48.0_f64 - t57040 / 48.0_f64 - t57042 / 384.0_f64 - t54199 + t57044 / 8.0_f64 - t57046 / 48.0_f64 - t57048 / 96.0_f64 + t57050 / 192.0_f64 + t57052 / 128.0_f64 - t57054 / 24.0_f64;
    t57056
}
