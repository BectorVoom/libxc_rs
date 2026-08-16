//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1332/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1332(t54052: f64, t54072: f64, t54043: f64, t54045: f64, t54048: f64, t54057: f64, t54059: f64, t54061: f64, t54063: f64, t54065: f64, t54067: f64, t54069: f64) -> f64 {
    let t55452 = 7.0_f64 / 96.0_f64 * t54052;
    let t55460 = 7.0_f64 / 72.0_f64 * t54072;
    let t55461 = t54043 / 12.0_f64 + t54045 / 192.0_f64 + t54048 / 32.0_f64 - t55452 - t54057 / 4.0_f64 - 5.0_f64 / 96.0_f64 * t54059 + t54061 / 48.0_f64 + t54063 / 192.0_f64 - t54065 / 96.0_f64 + t54067 / 96.0_f64 - t54069 / 16.0_f64 + t55460;
    t55461
}
