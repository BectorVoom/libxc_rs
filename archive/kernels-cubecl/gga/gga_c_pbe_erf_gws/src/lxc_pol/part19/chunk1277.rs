//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1277/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1277<F: Float>(t11443: F, t13917: F, t53138: F, t14583: F, t53496: F, t53841: F, t53923: F, t9942: F, t11354: F, t14797: F, t3989: F, t3990: F) -> (F, F, F, F) {
    let t56206 = t13917 * t53138 * t11443;
    let t56209 = t13917 * t53496 * t14583;
    let t56236 = t53923 * t53841 * t9942;
    let t56240 = t3989 * t3990 * t14797 * t11354;
    (t56206, t56209, t56236, t56240)
}
