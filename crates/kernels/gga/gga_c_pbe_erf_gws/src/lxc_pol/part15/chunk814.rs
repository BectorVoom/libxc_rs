//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 814/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk814<F: Float>(t3205: F, t336: F, t2182: F, t343: F, t2122: F, t337: F, t810: F, t2147: F, t2133: F, t2387: F, t2153: F, t837: F, t863: F) -> (F, F, F, F, F) {
    let t6523 = t3205 * t336;
    let t6524 = t343 * t2182;
    let t6534 = t337 * t2122 * t810;
    let t6535 = t2147 * t6534;
    let t6538 = t2387 * t2133;
    let t6542 = t863 * t2153 * t837;
    (t6523, t6524, t6535, t6538, t6542)
}
