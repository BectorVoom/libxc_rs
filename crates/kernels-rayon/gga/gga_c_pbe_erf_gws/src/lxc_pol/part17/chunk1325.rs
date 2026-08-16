//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1325/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1325(t4028: f64, t9013: f64, t1158: f64, t51395: f64, t14058: f64, t3268: f64, t1140: f64, t14083: f64, t3190: f64, t3206: f64, t2146: f64, t14007: f64, t9545: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t54350 = t4028 * t9013;
    let t54352 = t51395 * t1158;
    let t54354 = t14058 * t3268;
    let t54355 = 7.0_f64 / 288.0_f64 * t54354;
    let t54356 = t14083 * t1140;
    let t54359 = t3206 * t3190;
    let t54360 = t2146 * t54359;
    let t54362 = t14007 * t9545;
    (t54350, t54352, t54355, t54356, t54360, t54362)
}
