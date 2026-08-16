//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1100/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1100<F: Float>(t337: F, t6658: F, t6385: F, t831: F, t830: F, t2182: F, t353: F, t8599: F, t898: F, t938: F, t2387: F, t6792: F) -> (F, F, F) {
    let t19693 = t6658 * t337;
    let t19694 = t831 * t6385;
    let t19696 = t19693 * t830 * t19694;
    let t19701 = t8599 * t353 * t898 * t2182 * t938;
    let t19704 = t2387 * t6792;
    (t19696, t19701, t19704)
}
