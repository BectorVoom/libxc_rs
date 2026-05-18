//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 640/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk640<F: Float>(t1112: F, t328: F, t2306: F, t3074: F, t377: F, t858: F, t3065: F, t1114: F, t2366: F) -> (F, F, F, F, F, F) {
    let t3075 = t1112 * t328;
    let t3076 = t2306 * t3075;
    let t3077 = t3074 * t3076;
    let t3078 = t858 * t377;
    let t3079 = t3065 * t3078;
    let t3083 = t1114 * t2366;
    (t3075, t3076, t3077, t3078, t3079, t3083)
}
