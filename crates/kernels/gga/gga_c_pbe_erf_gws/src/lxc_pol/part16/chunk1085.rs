//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1085/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1085<F: Float>(t14617: F, t50943: F, t3989: F, t3990: F, t3991: F, t9080: F, t345: F, t6126: F, t9297: F, t14797: F, t8647: F, t13917: F, t13919: F, t9433: F, t13859: F, t9218: F) -> (F, F, F, F, F, F) {
    let t53272 = t50943 * t14617;
    let t53276 = t3989 * t3990 * t3991 * t9080;
    let t53283 = t345 * t6126;
    let t53286 = t3989 * t3990 * t53283 * t9297;
    let t53299 = t3989 * t3990 * t14797 * t8647;
    let t53323 = t13917 * t13919 * t9433;
    let t53327 = t13859 * t3990 * t14797 * t9218;
    (t53272, t53276, t53286, t53299, t53323, t53327)
}
