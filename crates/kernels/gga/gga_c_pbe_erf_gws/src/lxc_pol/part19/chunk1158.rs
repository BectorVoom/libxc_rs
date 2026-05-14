//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1158/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1158<F: Float>(t11585: F, t4028: F, t11693: F, t51274: F, t14058: F, t3875: F, t36666: F, t850: F, t14093: F, t11849: F, t14031: F, t11798: F, t12009: F, t14046: F, t15248: F, t11990: F, t338: F, t54244: F) -> (F, F, F, F, F, F, F, F, F) {
    let t57184 = t4028 * t11585;
    let t57186 = t51274 * t11693;
    let t57188 = t14058 * t3875;
    let t57190 = t850 * t36666;
    let t57191 = t57190 * t14093;
    let t57195 = t14031 * t11849;
    let t57197 = t14031 * t11798;
    let t57199 = t14031 * t12009;
    let t57201 = t14046 * t15248;
    let t57204 = t54244 * t338 * t11990;
    (t57184, t57186, t57188, t57191, t57195, t57197, t57199, t57201, t57204)
}
