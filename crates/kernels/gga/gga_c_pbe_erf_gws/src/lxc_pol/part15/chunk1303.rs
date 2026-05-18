//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1303/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1303<F: Float>(t14015: F, t9651: F, t9517: F, t14011: F, t9397: F, t3228: F, t51465: F, t14031: F, t9377: F, t3224: F, t1114: F, t51266: F) -> (F, F, F, F, F, F, F) {
    let t54107 = t14015 * t9651;
    let t54109 = t14015 * t9517;
    let t54111 = t14011 * t9397;
    let t54113 = t51465 * t3228;
    let t54114 = F::new(7.0) / F::new(288.0) * t54113;
    let t54115 = t14031 * t9377;
    let t54117 = t51465 * t3224;
    let t54118 = F::new(7.0) / F::new(288.0) * t54117;
    let t54119 = t1114 * t51266;
    (t54107, t54109, t54111, t54114, t54115, t54118, t54119)
}
