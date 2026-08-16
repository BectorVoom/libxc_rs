//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1182/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1182<F: Float>(t3810: F, t4039: F, t11628: F, t3139: F, t4028: F, t3862: F, t3975: F, t3972: F, t13780: F, t3742: F, t3990: F, t13859: F) -> (F, F, F, F, F, F, F) {
    let t15266 = t4039 * t3810;
    let t15268 = t3139 * t11628;
    let t15269 = t4028 * t15268;
    let t15278 = t3975 * t3862;
    let t15279 = t3972 * t15278;
    let t15282 = t3990 * t13780 * t3742;
    let t15283 = t13859 * t15282;
    (t15266, t15268, t15269, t15278, t15279, t15282, t15283)
}
