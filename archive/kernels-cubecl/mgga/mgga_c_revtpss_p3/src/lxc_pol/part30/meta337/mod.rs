//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta337 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1346;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1347;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1348;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1349;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1350;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta337<F: Float>(t11132: F, t2435: F, t907: F, t2854: F, t689: F, t2859: F, t2863: F, t159: F, t3181: F, t2851: F, t631: F, t45: F, t1071: F, t3057: F, t3259: F, t994: F, t342: F, t992: F, t338: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t11133, t11134) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1346::<F>(t11132, t2435, t907);
        let t11136 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1347::<F>(t2854, t689);
        let t11138 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1348::<F>(t2859, t689);
        let t11140 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1349::<F>(t2863, t689);
        let (t11142, t11144, t11150, t11187, t11190, t11195, t11200) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1350::<F>(t159, t3181, t2851, t631, t45, t1071, t3057, t3259, t994, t342, t992, t338);
    (t11133, t11134, t11136, t11138, t11140, t11142, t11144, t11150, t11187, t11190, t11195, t11200)
}
