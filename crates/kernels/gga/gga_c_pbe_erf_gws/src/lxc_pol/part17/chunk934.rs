//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 934/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk934<F: Float>(t2345: F, t875: F, t9404: F, t2253: F, t2277: F, t2343: F, t3247: F, t6637: F, t8878: F, t8883: F, t8889: F, t9366: F, t9372: F, t9377: F, t9382: F, t9389: F, t9393: F, t9397: F, t9401: F) -> (F, F) {
    let t9406 = t2345 * t9404 * t875;
    let t9409 = t2277 * t9366 / 384.0 - t8878 - t2253 * t9372 / 384.0 - t2253 * t9377 / 384.0 - t2253 * t9382 / 768.0 - t6637 * t9389 / 192.0 - t8883 - t2343 * t9393 / 1536.0 + t2343 * t9397 / 384.0 + t3247 * t9401 / 256.0 + t2343 * t9406 / 192.0 - t8889;
    (t9406, t9409)
}
