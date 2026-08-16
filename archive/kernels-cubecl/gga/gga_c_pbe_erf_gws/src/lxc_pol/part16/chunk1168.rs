//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1168/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1168<F: Float>(t3324: F, t4120: F, t360: F, t898: F, t2416: F, t2100: F, t376: F, t2219: F, t4383: F, t4408: F, t2387: F, t6792: F) -> (F, F, F, F, F, F, F) {
    let t15113 = t4120 * t3324;
    let t15636 = t898 * t360;
    let t15641 = t2416 * t360;
    let t19615 = t376 * t2100;
    let t19631 = t2219 * t898;
    let t19658 = t4408 * t4383;
    let t19704 = t2387 * t6792;
    (t15113, t15636, t15641, t19615, t19631, t19658, t19704)
}
