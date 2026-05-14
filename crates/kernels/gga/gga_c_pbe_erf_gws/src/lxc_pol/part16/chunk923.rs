//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 923/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk923<F: Float>(t1161: F, t2182: F, t2376: F, t2409: F, t1105: F, t2417: F, t3067: F, t1162: F, t2220: F, t338: F, t1144: F, t2402: F, t2418: F, t2231: F, t19: F, t931: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9213 = t1161 * t2182;
    let t9215 = t2409 * t2376 * t9213;
    let t9218 = t1105 * t2417;
    let t9220 = t2409 * t3067 * t9218;
    let t9224 = t338 * t2220 * t1162;
    let t9228 = t338 * t1144 * t2402;
    let t9232 = t338 * t1144 * t2418;
    let t9236 = t338 * t1144 * t2231;
    let t9239 = t931 * t19;
    (t9213, t9215, t9218, t9220, t9224, t9228, t9232, t9236, t9239)
}
