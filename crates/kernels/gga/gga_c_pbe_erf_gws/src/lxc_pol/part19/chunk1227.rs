//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1227/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1227<F: Float>(t3308: F, t859: F, t3973: F, t3991: F, t15641: F, t3098: F, t4386: F, t3316: F, t14125: F, t3111: F, t833: F, t850: F) -> (F, F, F, F, F, F) {
    let t53233 = t859 * t3308;
    let t53236 = t3973 * t3991;
    let t53240 = t3973 * t15641;
    let t53245 = t4386 * t3098;
    let t53250 = t859 * t3316;
    let t53260 = t850 * t3111 * t14125 * t833;
    (t53233, t53236, t53240, t53245, t53250, t53260)
}
