//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1322/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1322(t11516: f64, t14011: f64, t11934: f64, t51222: f64, t54053: f64, t54073: f64, t54088: f64, t55469: f64, t56910: f64, t56912: f64, t56914: f64, t56917: f64, t56920: f64, t56922: f64) -> f64 {
    let t56924 = t14011 * t11516;
    let t56926 = t14011 * t11934;
    let t56928 = 35.0_f64 / 432.0_f64 * t51222 + t56910 / 48.0_f64 - t54053 + t56912 / 192.0_f64 + t56914 / 24.0_f64 + t54073 + t56917 / 48.0_f64 - t56920 / 96.0_f64 + 7.0_f64 / 1152.0_f64 * t56922 + t54088 + t55469 + t56924 / 192.0_f64 - t56926 / 768.0_f64;
    t56928
}
