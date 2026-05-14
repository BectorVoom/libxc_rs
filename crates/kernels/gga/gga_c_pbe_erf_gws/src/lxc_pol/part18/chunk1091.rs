//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1091/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1091<F: Float>(t3316: F, t859: F, t1192: F, t20173: F, t14125: F, t3111: F, t833: F, t850: F, t14617: F, t50943: F, t345: F, t6126: F, t14669: F, t9270: F, t14448: F, t4414: F) -> (F, F, F, F, F, F, F) {
    let t53250 = t859 * t3316;
    let t53253 = t20173 * t1192;
    let t53260 = t850 * t3111 * t14125 * t833;
    let t53261 = 7.0 / 144.0 * t53260;
    let t53272 = t50943 * t14617;
    let t53273 = 7.0 / 144.0 * t53272;
    let t53283 = t345 * t6126;
    let t53302 = 7.0 / 72.0 * t9270 * t14669;
    let t53308 = 7.0 / 72.0 * t4414 * t14448;
    (t53250, t53253, t53261, t53273, t53283, t53302, t53308)
}
