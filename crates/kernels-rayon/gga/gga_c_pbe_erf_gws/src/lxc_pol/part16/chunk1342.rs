//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1342/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1342(t54319: f64, t54322: f64, t54329: f64, t51408: f64, t51412: f64, t52696: f64, t54315: f64, t54317: f64, t54324: f64, t54326: f64, t54333: f64, t54335: f64) -> f64 {
    let t55591 = 7.0_f64 / 36.0_f64 * t54319;
    let t55593 = 7.0_f64 / 36.0_f64 * t54322;
    let t55596 = 7.0_f64 / 12.0_f64 * t54329;
    let t55600 = t54315 / 12.0_f64 + t54317 / 12.0_f64 - t55591 - 35.0_f64 / 108.0_f64 * t51408 - t55593 - t54324 / 48.0_f64 - t54326 / 96.0_f64 - t55596 - 35.0_f64 / 54.0_f64 * t51412 - t52696 + t54333 / 8.0_f64 - t54335 / 192.0_f64;
    t55600
}
