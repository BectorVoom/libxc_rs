//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 854/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk854<F: Float>(t21: F, t5589: F, t2719: F, t1041: F, t1251: F, t1691: F, t7093: F, t11: F, t7212: F, t2704: F, t7097: F, t1413: F, t2678: F) -> (F, F, F, F, F, F, F) {
    let t7236 = t21 * t5589;
    let t7237 = t7236 * t2719;
    let t7239 = t1251 * t1041;
    let t7248 = t1691 * t7093;
    let t7249 = t11 * t7248;
    let t7251 = t1691 * t7212;
    let t7252 = t2704 * t7251;
    let t7254 = t1691 * t7097;
    let t7255 = t11 * t7254;
    let t7257 = t2678 * t1413;
    (t7236, t7237, t7239, t7249, t7252, t7255, t7257)
}
