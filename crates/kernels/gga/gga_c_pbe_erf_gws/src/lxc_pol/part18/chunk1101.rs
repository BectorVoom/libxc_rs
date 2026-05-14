//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1101/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1101<F: Float>(t53896: F, t2370: F, t3958: F, t1144: F, t13923: F, t859: F, t13911: F, t26958: F, t22336: F, t4002: F, t14784: F, t50994: F, t20091: F, t4157: F, t3202: F, t3955: F) -> (F, F, F, F, F, F, F, F) {
    let t53897 = 7.0 / 72.0 * t53896;
    let t53923 = t3958 * t2370;
    let t53939 = t859 * t1144 * t13923;
    let t53943 = 7.0 / 72.0 * t26958 * t13911;
    let t53948 = 7.0 / 144.0 * t22336 * t4002;
    let t53952 = t50994 * t14784;
    let t53953 = 7.0 / 288.0 * t53952;
    let t53959 = t20091 * t4157;
    let t53970 = t3955 * t3202;
    (t53897, t53923, t53939, t53943, t53948, t53953, t53959, t53970)
}
