//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1320/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1320<F: Float>(t11589: F, t4039: F, t14028: F, t3810: F, t11480: F, t4028: F, t14547: F, t20842: F, t38545: F, t37454: F, t6523: F, t11461: F) -> (F, F, F, F, F, F) {
    let t57119 = t4039 * t11589;
    let t57121 = t14028 * t3810;
    let t57123 = t4028 * t11480;
    let t57127 = t14547 * t20842 * t38545;
    let t57130 = t14547 * t6523 * t37454;
    let t57132 = t4028 * t11461;
    (t57119, t57121, t57123, t57127, t57130, t57132)
}
