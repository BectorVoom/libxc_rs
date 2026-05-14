//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1086/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1086<F: Float>(t3973: F, t3991: F, t15641: F, t3098: F, t4386: F, t3316: F, t859: F, t14125: F, t3111: F, t833: F, t850: F, t14617: F, t50943: F, t345: F, t6126: F, t4146: F, t51818: F) -> (F, F, F, F, F, F, F, F) {
    let t53236 = t3973 * t3991;
    let t53240 = t3973 * t15641;
    let t53245 = t4386 * t3098;
    let t53250 = t859 * t3316;
    let t53260 = t850 * t3111 * t14125 * t833;
    let t53272 = t50943 * t14617;
    let t53283 = t345 * t6126;
    let t53334 = t51818 * t4146;
    (t53236, t53240, t53245, t53250, t53260, t53272, t53283, t53334)
}
