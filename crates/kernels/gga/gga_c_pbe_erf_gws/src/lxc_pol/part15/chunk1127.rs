//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1127/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1127<F: Float>(t51819: F, t2370: F, t3958: F, t53841: F, t9284: F, t4149: F, t50998: F, t9505: F, t13917: F, t3223: F, t361: F, t52921: F, t1144: F, t13923: F, t859: F, t13911: F, t26958: F) -> (F, F, F, F, F, F) {
    let t53915 = 119.0 / 6912.0 * t51819;
    let t53923 = t3958 * t2370;
    let t53925 = t53923 * t53841 * t9284;
    let t53930 = t50998 * t4149 * t9505;
    let t53936 = t13917 * t361 * t52921 * t3223;
    let t53939 = t859 * t1144 * t13923;
    let t53943 = 7.0 / 72.0 * t26958 * t13911;
    (t53915, t53925, t53930, t53936, t53939, t53943)
}
