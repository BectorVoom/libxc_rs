//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta66 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk426;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk427;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk428;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk429;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk430;
use chunk5::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk431;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta66<F: Float>(t1269: F, t225: F, t494: F, t460: F, t487: F, t493: F, t473: F, t1214: F, t1032: F, t1243: F, t355: F, t471: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t1271, t1274) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk426::<F>(t1269, t225, t494, t460, t487);
        let (t1275, t1276, t1277) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk427::<F>(t493, t225);
        let t1280 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk428::<F>(t473, t487);
        let (t1281, t1284) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk429::<F>(t1214, t1280, t1032, t1243);
        let t1285 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk430::<F>(t1284, t460);
        let t1287 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk431::<F>(t355, t471);
    (t1271, t1274, t1275, t1276, t1277, t1280, t1281, t1284, t1285, t1287)
}
