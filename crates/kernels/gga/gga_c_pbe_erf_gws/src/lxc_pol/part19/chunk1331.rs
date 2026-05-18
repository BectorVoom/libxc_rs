//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1331/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1331<F: Float>(t13953: F, t15164: F, t11547: F, t13917: F, t53156: F, t15296: F, t3979: F, t11356: F, t3965: F, t14121: F, t2409: F, t39579: F) -> (F, F, F, F, F) {
    let t57358 = t13953 * t15164;
    let t57361 = t13917 * t53156 * t11547;
    let t57371 = t3979 * t15296;
    let t57375 = t3965 * t11356;
    let t57379 = t14121 * t2409 * t39579;
    (t57358, t57361, t57371, t57375, t57379)
}
