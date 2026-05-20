//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta168 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk850;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk851;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk852;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk853;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk854;
use chunk5::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk855;
use chunk6::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk856;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta168<F: Float>(t3089: F, t3623: F, t1285: F, t1264: F, t828: F, t1248: F, t73: F, t1121: F, t471: F, t606: F, t126: F, t1263: F, t1122: F, t247: F, t1261: F, t3372: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t3624 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk850::<F>(t3089, t3623);
        let t3625 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk851::<F>(t1285, t3624);
        let t3626 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk852::<F>(t1264, t828);
        let (t3627, t3628, t3629, t3630, t3631) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk853::<F>(t1248, t73, t1121, t471, t606, t3626);
        let t3634 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk854::<F>(t126, t1263);
        let t3636 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk855::<F>(t1122, t3634, t247);
        let (t3637, t3640) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk856::<F>(t1261, t3636, t1264, t3372, t247);
    (t3624, t3625, t3626, t3627, t3628, t3629, t3630, t3631, t3634, t3636, t3637, t3640)
}
