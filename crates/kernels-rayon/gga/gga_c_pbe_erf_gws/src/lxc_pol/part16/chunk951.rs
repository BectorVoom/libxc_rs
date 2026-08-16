//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 951/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk951(t7228: f64, t7230: f64, t7317: f64, t7319: f64, t7321: f64, t7324: f64, t7416: f64, t7417: f64, t7418: f64, t7419: f64, t7420: f64, t7422: f64, t7423: f64, t7424: f64, t7427: f64, t7431: f64, t7434: f64) -> f64 {
    let t8428 = -t7228 + t7230 - t7317 - t7319 + t7321 - t7324 - t7416 - t7417 - t7418 + t7419 + t7420 - t7422 - t7423 - t7424 + t7427 + t7431 + t7434;
    t8428
}
