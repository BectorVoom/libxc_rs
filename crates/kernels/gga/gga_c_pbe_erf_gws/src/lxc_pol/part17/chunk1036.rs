//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1036/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1036<F: Float>(t360: F, t898: F, t2416: F, t2100: F, t376: F, t2219: F, t4383: F, t4408: F, t2387: F, t6792: F, t2365: F, t56: F, t2118: F, t822: F, t2306: F, t2382: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t15636 = t898 * t360;
    let t15641 = t2416 * t360;
    let t19615 = t376 * t2100;
    let t19631 = t2219 * t898;
    let t19658 = t4408 * t4383;
    let t19704 = t2387 * t6792;
    let t19775 = t2365 * t56;
    let t19776 = t2118 * t19775;
    let t19777 = t822 * t19776;
    let t19894 = t2306 * t4383;
    let t19895 = t2382 * t19894;
    (t15636, t15641, t19615, t19631, t19658, t19704, t19775, t19776, t19777, t19895)
}
