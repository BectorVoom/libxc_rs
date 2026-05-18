//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1248/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1248<F: Float>(t14797: F, t3989: F, t3990: F, t8647: F, t14669: F, t9270: F, t14448: F, t4414: F, t13917: F, t13919: F, t9433: F, t13859: F, t9218: F) -> (F, F, F, F, F) {
    let t53299 = t3989 * t3990 * t14797 * t8647;
    let t53302 = F::new(7.0) / F::new(72.0) * t9270 * t14669;
    let t53308 = F::new(7.0) / F::new(72.0) * t4414 * t14448;
    let t53323 = t13917 * t13919 * t9433;
    let t53327 = t13859 * t3990 * t14797 * t9218;
    (t53299, t53302, t53308, t53323, t53327)
}
