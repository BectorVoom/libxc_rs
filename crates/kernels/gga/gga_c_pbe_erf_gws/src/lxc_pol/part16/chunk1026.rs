//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1026/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1026<F: Float>(t12213: F, t2409: F, t4097: F, t4207: F, t6781: F, t1205: F, t8589: F, t829: F, t830: F, t3083: F, t4083: F, t2376: F, t4227: F, t2494: F, t1144: F, t338: F, t4111: F) -> (F, F, F, F, F, F, F, F) {
    let t14902 = t2409 * t12213 * t4097;
    let t14906 = t2409 * t6781 * t4207;
    let t14909 = t8589 * t1205;
    let t14911 = t829 * t830 * t14909;
    let t14914 = t3083 * t4083;
    let t14916 = t2376 * t4227;
    let t14918 = t829 * t830 * t14916;
    let t14922 = t1205 * t2494;
    let t14924 = t2409 * t2376 * t14922;
    let t14928 = t338 * t1144 * t4111;
    (t14902, t14906, t14911, t14914, t14918, t14922, t14924, t14928)
}
