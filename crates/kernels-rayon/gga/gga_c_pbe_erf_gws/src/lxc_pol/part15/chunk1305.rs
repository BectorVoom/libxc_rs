//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1305/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1305(t54107: f64, t54109: f64, t54111: f64, t54114: f64, t54115: f64, t54118: f64, t54120: f64, t54122: f64, t54124: f64, t54126: f64, t54129: f64, t54130: f64) -> f64 {
    let t54132 = t54107 / 96.0_f64 - t54109 / 48.0_f64 + t54111 / 192.0_f64 + t54114 - t54115 / 192.0_f64 + t54118 + t54120 / 48.0_f64 - t54122 / 48.0_f64 + t54124 / 192.0_f64 + 119.0_f64 / 3456.0_f64 * t54126 - t54129 + t54130 / 96.0_f64;
    t54132
}
