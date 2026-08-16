//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1250/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1250<F: Float>(t13781: F, t14582: F, t3972: F, t9550: F, t14592: F, t50994: F, t14657: F, t6797: F, t14136: F, t8690: F, t2112: F, t2306: F, t3975: F, t9385: F) -> (F, F, F, F, F) {
    let t53351 = t3972 * t13781 * t14582 * t9550;
    let t53353 = t50994 * t14592;
    let t53354 = F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t53353;
    let t53355 = t14657 * t6797;
    let t53357 = t14136 * t8690;
    let t53362 = t3972 * t3975 * t9385 * t2306 * t2112;
    (t53351, t53354, t53355, t53357, t53362)
}
