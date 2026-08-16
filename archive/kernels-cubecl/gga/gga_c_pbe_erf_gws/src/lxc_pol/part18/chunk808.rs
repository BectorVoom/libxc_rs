//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 808/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk808<F: Float>(t19: F, t6658: F, t2132: F, t328: F, t824: F, t822: F, t2118: F, t2263: F, t358: F, t356: F, t2252: F, t346: F, t4408: F) -> (F, F, F, F, F, F, F, F) {
    let t6659 = t6658 * t19;
    let t6670 = t2132 * t328;
    let t6671 = t824 * t6670;
    let t6672 = t822 * t6671;
    let t6677 = t2118 * t6670;
    let t6683 = t358 * t2263;
    let t6684 = t356 * t6683;
    let t6685 = t6684 * t2252;
    let t6701 = t4408 * t346;
    (t6659, t6671, t6672, t6677, t6683, t6684, t6685, t6701)
}
