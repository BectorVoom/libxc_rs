//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1086/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1086<F: Float>(t4146: F, t51818: F, t14797: F, t3989: F, t3990: F, t9321: F, t13781: F, t14582: F, t3972: F, t9380: F, t9550: F, t14592: F, t50994: F, t14657: F, t6797: F, t14136: F, t8690: F) -> (F, F, F, F, F, F, F) {
    let t53334 = t51818 * t4146;
    let t53338 = t3989 * t3990 * t14797 * t9321;
    let t53346 = t3972 * t13781 * t14582 * t9380;
    let t53351 = t3972 * t13781 * t14582 * t9550;
    let t53353 = t50994 * t14592;
    let t53355 = t14657 * t6797;
    let t53357 = t14136 * t8690;
    (t53334, t53338, t53346, t53351, t53353, t53355, t53357)
}
