//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1249/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1249<F: Float>(t1114: F, t51922: F, t14001: F, t3214: F, t2370: F, t3958: F, t1144: F, t13923: F, t859: F, t13911: F, t26958: F, t22336: F, t4002: F) -> (F, F, F, F, F, F) {
    let t53891 = t1114 * t51922;
    let t53896 = t14001 * t3214;
    let t53897 = F::new(7.0) / F::new(72.0) * t53896;
    let t53923 = t3958 * t2370;
    let t53939 = t859 * t1144 * t13923;
    let t53943 = F::new(7.0) / F::new(72.0) * t26958 * t13911;
    let t53948 = F::new(7.0) / F::new(144.0) * t22336 * t4002;
    (t53891, t53897, t53923, t53939, t53943, t53948)
}
