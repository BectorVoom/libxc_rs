//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 821/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk821<F: Float>(t56: F, t931: F, t19: F, t274: F, t6161: F, t2132: F, t328: F, t824: F, t822: F, t6277: F, t858: F, t2407: F) -> (F, F, F, F, F, F) {
    let t6658 = t56 * t931;
    let t6659 = t6658 * t19;
    let t6665 = t274 * t6161;
    let t6670 = t2132 * t328;
    let t6671 = t824 * t6670;
    let t6672 = t822 * t6671;
    let t6673 = t858 * t6277;
    let t6674 = t2407 * t6673;
    (t6659, t6665, t6670, t6671, t6672, t6674)
}
