//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1101/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1101<F: Float>(t1113: F, t28647: F, t3972: F, t3975: F, t13776: F, t38360: F, t13781: F, t51134: F, t1118: F, t3223: F, t361: F, t50998: F, t874: F, t1112: F, t13918: F, t13917: F, t6639: F) -> (F, F, F, F, F, F) {
    let t53432 = t3972 * t3975 * t1113 * t28647;
    let t53435 = t13776 * t3975 * t38360;
    let t53439 = t3972 * t13781 * t1113 * t51134;
    let t53444 = t50998 * t361 * t1118 * t874 * t3223;
    let t53446 = t13918 * t1112;
    let t53447 = t361 * t53446;
    let t53449 = t13917 * t53447 * t6639;
    (t53432, t53435, t53439, t53444, t53447, t53449)
}
