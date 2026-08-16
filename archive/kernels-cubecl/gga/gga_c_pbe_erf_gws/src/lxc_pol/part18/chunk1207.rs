//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1207/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1207<F: Float>(t2416: F, t3199: F, t326: F, t825: F, t6148: F, t3067: F, t830: F, t3916: F, t6792: F, t11609: F, t2118: F, t2494: F, param_a_c: F) -> (F, F, F, F, F, F, F) {
    let t36129 = t3199 * t2416;
    let t36199 = t326 * t825;
    let t36200 = t36199 * t6148;
    let t36201 = t830 * t3067;
    let t36323 = t3916 * t6792;
    let t36666 = t2118 * t11609;
    let t36888 = t2494 * param_a_c;
    (t36129, t36199, t36200, t36201, t36323, t36666, t36888)
}
