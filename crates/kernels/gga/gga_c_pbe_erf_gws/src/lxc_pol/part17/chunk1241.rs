//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1241/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1241<F: Float>(t14452: F, t9270: F, t14759: F, t4414: F, t14633: F, t51666: F, t13780: F, t14637: F, t3990: F, t9213: F, t13859: F, t9702: F) -> (F, F, F, F, F) {
    let t53187 = F::new(7.0) / F::new(72.0) * t9270 * t14452;
    let t53189 = F::new(7.0) / F::new(72.0) * t4414 * t14759;
    let t53198 = t51666 * t14633;
    let t53199 = F::new(7.0) / F::new(576.0) * t53198;
    let t53207 = t14637 * t3990 * t13780 * t9213;
    let t53212 = t13859 * t3990 * t13780 * t9702;
    (t53187, t53189, t53199, t53207, t53212)
}
