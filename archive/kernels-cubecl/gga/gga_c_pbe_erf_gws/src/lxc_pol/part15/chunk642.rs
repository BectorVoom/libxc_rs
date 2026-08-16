//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 642/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk642<F: Float>(t1144: F, t845: F, t338: F, t1118: F, t892: F, t2494: F, t376: F, t353: F, t1162: F, t1112: F, t339: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3089 = t1144 * t845;
    let t3090 = t338 * t3089;
    let t3093 = t892 * t1118;
    let t3094 = t338 * t3093;
    let t3097 = t376 * t2494;
    let t3098 = t353 * t3097;
    let t3099 = t338 * t3098;
    let t3102 = t892 * t1162;
    let t3103 = t338 * t3102;
    let t3106 = t1112 * t339;
    (t3089, t3090, t3093, t3094, t3097, t3098, t3099, t3102, t3103, t3106)
}
