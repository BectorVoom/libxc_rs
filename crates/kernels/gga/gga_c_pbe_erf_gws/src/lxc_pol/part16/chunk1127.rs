//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1127/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1127<F: Float>(t14031: F, t9382: F, t9552: F, t4028: F, t9116: F, t4142: F, t51529: F, t13953: F, t14648: F, t13796: F, t14724: F, t2352: F, t343: F, t3989: F, t13972: F, t14684: F) -> (F, F, F, F, F, F, F) {
    let t54406 = t14031 * t9382;
    let t54408 = t14031 * t9552;
    let t54411 = t4028 * t9116;
    let t54427 = t51529 * t4142;
    let t54429 = t13953 * t14648;
    let t54461 = t3989 * t13796 * t14724 * t343 * t2352;
    let t54463 = t13972 * t14684;
    (t54406, t54408, t54411, t54427, t54429, t54461, t54463)
}
