//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1334/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1334(t54113: f64, t54117: f64, t54126: f64, t54128: f64, t54107: f64, t54109: f64, t54111: f64, t54115: f64, t54120: f64, t54122: f64, t54124: f64, t54130: f64) -> f64 {
    let t55480 = 7.0_f64 / 144.0_f64 * t54113;
    let t55482 = 7.0_f64 / 144.0_f64 * t54117;
    let t55486 = 119.0_f64 / 1728.0_f64 * t54126;
    let t55487 = 7.0_f64 / 288.0_f64 * t54128;
    let t55489 = t54107 / 48.0_f64 - t54109 / 24.0_f64 + t54111 / 96.0_f64 + t55480 - t54115 / 96.0_f64 + t55482 + t54120 / 24.0_f64 - t54122 / 24.0_f64 + t54124 / 96.0_f64 + t55486 - t55487 + t54130 / 48.0_f64;
    t55489
}
