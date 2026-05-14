//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1156/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1156<F: Float>(t15240: F, t54079: F, t54230: F, t11451: F, t14031: F, t54119: F, t8983: F, t3108: F, t3133: F, t54253: F, t11625: F, t14007: F, t11521: F, t14498: F, t11930: F, t14015: F) -> (F, F, F, F, F, F, F, F) {
    let t56883 = t54079 * t15240;
    let t56885 = t54230 * t15240;
    let t56887 = t14031 * t11451;
    let t56889 = t54119 * t8983;
    let t56892 = t3108 * t54253 * t3133;
    let t56894 = t14007 * t11625;
    let t56896 = t14498 * t11521;
    let t56898 = t14015 * t11930;
    (t56883, t56885, t56887, t56889, t56892, t56894, t56896, t56898)
}
