//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 682/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk682<F: Float>(t1195: F, t840: F, t1192: F, t810: F, t2376: F, t2409: F, t1193: F, t892: F, t338: F, t938: F, t3067: F, t331: F, t345: F, t56: F, t859: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4006 = 7.0 / 288.0 * t840 * t1195;
    let t4007 = t1192 * t810;
    let t4009 = t2409 * t2376 * t4007;
    let t4012 = t892 * t1193;
    let t4013 = t338 * t4012;
    let t4016 = t1192 * t938;
    let t4018 = t2409 * t3067 * t4016;
    let t4021 = t345 * t331;
    let t4022 = t4021 * t56;
    let t4023 = t4022 * t859;
    (t4006, t4007, t4009, t4013, t4016, t4018, t4021, t4022, t4023)
}
