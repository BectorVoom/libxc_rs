//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1348/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1348(t11849: f64, t14031: f64, t11798: f64, t12009: f64, t14046: f64, t15248: f64, t11990: f64, t338: f64, t54244: f64, t14024: f64, t3805: f64, t11644: f64, t4028: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t57195 = t14031 * t11849;
    let t57197 = t14031 * t11798;
    let t57199 = t14031 * t12009;
    let t57201 = t14046 * t15248;
    let t57204 = t54244 * t338 * t11990;
    let t57206 = t3805 * t14024;
    let t57208 = t4028 * t11644;
    (t57195, t57197, t57199, t57201, t57204, t57206, t57208)
}
