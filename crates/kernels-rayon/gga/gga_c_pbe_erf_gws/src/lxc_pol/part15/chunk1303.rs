//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1303/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1303(t14015: f64, t9651: f64, t9517: f64, t14011: f64, t9397: f64, t3228: f64, t51465: f64, t14031: f64, t9377: f64, t3224: f64, t1114: f64, t51266: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t54107 = t14015 * t9651;
    let t54109 = t14015 * t9517;
    let t54111 = t14011 * t9397;
    let t54113 = t51465 * t3228;
    let t54114 = 7.0_f64 / 288.0_f64 * t54113;
    let t54115 = t14031 * t9377;
    let t54117 = t51465 * t3224;
    let t54118 = 7.0_f64 / 288.0_f64 * t54117;
    let t54119 = t1114 * t51266;
    (t54107, t54109, t54111, t54114, t54115, t54118, t54119)
}
