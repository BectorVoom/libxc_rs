//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1273/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1273(t14031: f64, t9377: f64, t3224: f64, t51465: f64, t1114: f64, t51266: f64, t6680: f64, t2134: f64, t8996: f64, t14015: f64, t9522: f64, t1150: f64, t51200: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t54115 = t14031 * t9377;
    let t54117 = t51465 * t3224;
    let t54119 = t1114 * t51266;
    let t54120 = t54119 * t6680;
    let t54122 = t2134 * t8996;
    let t54124 = t14015 * t9522;
    let t54126 = t51200 * t1150;
    (t54115, t54117, t54120, t54122, t54124, t54126)
}
