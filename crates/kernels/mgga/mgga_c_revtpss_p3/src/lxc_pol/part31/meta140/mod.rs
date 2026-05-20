//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta140 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk745;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk746;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk747;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk748;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk749;
use chunk5::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk750;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta140<F: Float>(t3617: F, t66: F, t474: F, t479: F, t3089: F, t1285: F, t1264: F, t828: F, t1248: F, t73: F, t1121: F, t471: F, t606: F, t126: F, t1263: F, t1122: F, t247: F, t1261: F, t1230: F, t1260: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3618, t3623) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk745::<F>(t3617, t66, t474, t479);
        let t3624 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk746::<F>(t3089, t3623);
        let t3625 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk747::<F>(t1285, t3624);
        let t3626 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk748::<F>(t1264, t828);
        let (t3627, t3628, t3629, t3634) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk749::<F>(t1248, t73, t1121, t471, t606, t126, t1263);
        let (t3636, t3637, t3647) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk750::<F>(t1122, t3634, t247, t1261, t1230, t1260);
    (t3618, t3623, t3624, t3625, t3626, t3627, t3628, t3629, t3634, t3636, t3637, t3647)
}
