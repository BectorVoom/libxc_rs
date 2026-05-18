//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 391/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk391<F: Float>(t27: F, t387: F, t13: F, t403: F, t404: F) -> (F, F, F, F, F) {
    let t1272 = t387 * t27;
    let t1273 = F::new(1.0) / t1272;
    let t1274 = t13 * t1273;
    let t1275 = t403 * t403;
    let t1276 = t1275 * t404;
    let t1277 = t1274 * t1276;
    let t1278 = F::new(2.0) * t1277;
    (t1273, t1274, t1275, t1276, t1278)
}
