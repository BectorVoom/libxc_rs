//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1277/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1277<F: Float>(t14657: F, t53250: F, t1134: F, t13776: F, t3060: F, t50956: F, t13859: F, t52926: F, t9942: F, t1109: F, t1192: F, t11443: F, t13917: F, t53138: F) -> (F, F, F, F, F) {
    let t56190 = t14657 * t53250;
    let t56194 = t13776 * t50956 * t1134 * t3060;
    let t56197 = t13859 * t52926 * t9942;
    let t56199 = t1192 * t1109;
    let t56206 = t13917 * t53138 * t11443;
    (t56190, t56194, t56197, t56199, t56206)
}
