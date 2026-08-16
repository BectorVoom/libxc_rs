//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1295/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1295<F: Float>(t1123: F, t3752: F, t50998: F, t51021: F, t938: F, t1134: F, t3068: F, t3972: F, t53240: F, t3902: F, t4386: F, t13792: F) -> (F, F, F) {
    let t56505 = t50998 * t51021 * t1123 * t3752 * t938;
    let t56511 = t3972 * t53240 * t1134 * t3068;
    let t56513 = t4386 * t3902;
    let t56514 = t13792 * t56513;
    (t56505, t56511, t56514)
}
