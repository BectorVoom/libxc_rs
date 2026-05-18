//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 323/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk323<F: Float>(t1231: F, t441: F, t126: F, t19: F, t299: F, t799: F, t119: F, t331: F, t391: F, t4: F, t542: F) -> (F, F, F, F, F, F, F, F) {
    let t1232 = t1231 * t441;
    let t1235 = F::new(1.0) / t126;
    let t1236 = t1235 * t19;
    let t1237 = t799 * t299;
    let t1238 = t1236 * t1237;
    let t1240 = t119 * t331;
    let t1241 = t391 * t1240;
    let t1243 = t4 * t542;
    (t1232, t1235, t1236, t1237, t1238, t1240, t1241, t1243)
}
