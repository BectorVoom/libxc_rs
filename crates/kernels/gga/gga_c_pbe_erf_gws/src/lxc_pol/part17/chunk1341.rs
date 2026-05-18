//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1341/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1341<F: Float>(t2370: F, t36199: F, t830: F, t9296: F, t51555: F, t53236: F, t8891: F, t14617: F, t50884: F, t22172: F, t2409: F, t3965: F) -> (F, F, F, F, F) {
    let t54598 = t36199 * t2370;
    let t54599 = t830 * t9296;
    let t54605 = t51555 * t53236 * t8891;
    let t54607 = t50884 * t14617;
    let t54613 = t3965 * t2409 * t22172;
    (t54598, t54599, t54605, t54607, t54613)
}
