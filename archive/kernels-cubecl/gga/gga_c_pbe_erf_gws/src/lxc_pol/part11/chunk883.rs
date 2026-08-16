//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 883/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk883<F: Float>(t1339: F, t16451: F, t1971: F, t1331: F, t8: F, t147: F, t551: F, t553: F, t6041: F, t6047: F, t1472: F, t2003: F, t671: F) -> (F, F, F, F, F, F, F) {
    let t16454 = F::cast_from(0.29725654166942986832e-2_f64) * t1339 * t16451 * t1971;
    let t16463 = F::cast_from(1.0_f64) / t8 / t1331;
    let t16465 = t16463 * t147 * t551;
    let t16467 = F::cast_from(0.74395492895254307406e-5_f64) * t16465 * t553;
    let t16471 = F::cast_from(0.1035981803916141664e0_f64) * t6041 * t553;
    let t16480 = F::cast_from(0.12408369628826103546e0_f64) * t6047 * t553;
    let t16485 = F::cast_from(0.19878653761973934499e-1_f64) * t2003 * t1472 * t671;
    (t16454, t16463, t16465, t16467, t16471, t16480, t16485)
}
