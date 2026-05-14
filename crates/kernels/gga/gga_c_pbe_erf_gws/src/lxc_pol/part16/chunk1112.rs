//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1112/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1112<F: Float>(t14011: F, t9393: F, t14498: F, t9401: F, t3179: F, t51291: F, t854: F, t14015: F, t9651: F, t9517: F, t9397: F, t3228: F, t51465: F, t14031: F, t9377: F, t3224: F) -> (F, F, F, F, F, F, F, F, F) {
    let t54096 = t14011 * t9393;
    let t54098 = t14498 * t9401;
    let t54101 = t51291 * t3179;
    let t54102 = t854 * t54101;
    let t54107 = t14015 * t9651;
    let t54109 = t14015 * t9517;
    let t54111 = t14011 * t9397;
    let t54113 = t51465 * t3228;
    let t54115 = t14031 * t9377;
    let t54117 = t51465 * t3224;
    (t54096, t54098, t54102, t54107, t54109, t54111, t54113, t54115, t54117)
}
