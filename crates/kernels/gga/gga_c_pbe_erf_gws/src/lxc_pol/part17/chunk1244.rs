//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1244/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1244<F: Float>(t3308: F, t859: F, t13792: F, t3973: F, t3991: F, t13776: F, t8828: F, t15641: F, t3068: F, t3972: F, t875: F, t3098: F, t4386: F) -> (F, F, F, F, F) {
    let t53233 = t859 * t3308;
    let t53234 = t13792 * t53233;
    let t53236 = t3973 * t3991;
    let t53238 = t13776 * t53236 * t8828;
    let t53240 = t3973 * t15641;
    let t53243 = t3972 * t53240 * t3068 * t875;
    let t53245 = t4386 * t3098;
    (t53234, t53236, t53238, t53243, t53245)
}
