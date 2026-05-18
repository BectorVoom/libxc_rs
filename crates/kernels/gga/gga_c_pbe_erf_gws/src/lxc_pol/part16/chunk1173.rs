//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1173/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1173<F: Float>(t22334: F, t2306: F, t3074: F, t3039: F, t4384: F, t6792: F, t2395: F, t2494: F, t1105: F, t4417: F, t1114: F, t19776: F) -> (F, F, F, F, F, F) {
    let t22336 = t3074 * t2306 * t22334;
    let t22343 = t3039 * t4384;
    let t22379 = t3039 * t6792;
    let t22393 = t2395 * t2494;
    let t22410 = t4417 * t1105;
    let t22493 = t1114 * t19776;
    (t22336, t22343, t22379, t22393, t22410, t22493)
}
