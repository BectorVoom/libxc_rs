//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1090/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1090<F: Float>(t53198: F, t13888: F, t3306: F, t353: F, t859: F, t14404: F, t19906: F, t1114: F, t50942: F, t13984: F, t3308: F, t3973: F, t3991: F, t15641: F, t3098: F, t4386: F) -> (F, F, F, F, F, F, F, F, F) {
    let t53199 = 7.0 / 576.0 * t53198;
    let t53220 = t859 * t353 * t13888 * t3306;
    let t53224 = 7.0 / 72.0 * t19906 * t14404;
    let t53229 = t1114 * t50942;
    let t53230 = t53229 * t13984;
    let t53231 = 7.0 / 144.0 * t53230;
    let t53233 = t859 * t3308;
    let t53236 = t3973 * t3991;
    let t53240 = t3973 * t15641;
    let t53245 = t4386 * t3098;
    (t53199, t53220, t53224, t53229, t53231, t53233, t53236, t53240, t53245)
}
