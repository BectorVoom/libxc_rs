//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1060/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1060<F: Float>(t15437: F, t15503: F, t15525: F, t15565: F, t1167: F, t15101: F, t14368: F, t3931: F, t3928: F, t4120: F, t360: F, t898: F, t2416: F, t4383: F, t4408: F, t2365: F, t56: F) -> (F, F, F, F, F, F, F, F) {
    let t15567 = t15437 + t15503 + t15525 + t15565;
    let t15571 = t15101 * t1167;
    let t15574 = t14368 * t3931;
    let t15577 = t4120 * t3928;
    let t15636 = t898 * t360;
    let t15641 = t2416 * t360;
    let t19658 = t4408 * t4383;
    let t19775 = t2365 * t56;
    (t15567, t15571, t15574, t15577, t15636, t15641, t19658, t19775)
}
