//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta166 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk840;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk841;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk842;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk843;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk844;
use chunk5::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk845;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta166<F: Float>(t3378: F, t3381: F, t3388: F, t3430: F, t3438: F, t3528: F, t3530: F, t3533: F, t3537: F, t3541: F, t3545: F, t1250: F, t482: F, t1042: F, t3140: F, t460: F, t1242: F, t472: F, t474: F, t3147: F, t479: F, t1248: F, t471: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t3588 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk840::<F>(t3378, t3381, t3388, t3430, t3438, t3528, t3530, t3533, t3537, t3541, t3545);
        let (t3590, t3591) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk841::<F>(t1250, t3588, t482, t1042);
        let t3594 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk842::<F>(t3140, t460);
        let t3596 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk843::<F>(t1242, t472);
        let (t3597, t3598, t3599, t3600, t3601) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk844::<F>(t3596, t474, t3147, t479, t3594, t1248);
        let (t3602, t3603) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk845::<F>(t3601, t482, t471);
    (t3588, t3590, t3591, t3594, t3596, t3597, t3598, t3599, t3600, t3601, t3602, t3603)
}
