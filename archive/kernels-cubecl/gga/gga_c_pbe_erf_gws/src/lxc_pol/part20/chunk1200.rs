//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1200/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1200<F: Float>(t3039: F, t4384: F, t6792: F, t1114: F, t19776: F, t2200: F, t857: F, t329: F, t6126: F, t891: F, t19658: F, t2409: F, t3205: F) -> (F, F, F, F, F, F, F) {
    let t22343 = t3039 * t4384;
    let t22379 = t3039 * t6792;
    let t22493 = t1114 * t19776;
    let t22508 = t2200 * t857;
    let t22509 = t329 * t22508;
    let t22534 = t891 * t6126;
    let t26604 = t1114 * t19658;
    let t26617 = t3205 * t2409;
    (t22343, t22379, t22493, t22509, t22534, t26604, t26617)
}
