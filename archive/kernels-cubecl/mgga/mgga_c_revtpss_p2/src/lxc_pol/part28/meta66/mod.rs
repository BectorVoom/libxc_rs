//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta66 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk424;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk425;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk426;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk427;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk428;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk429;
use chunk6::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk430;
use chunk7::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk431;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta66<F: Float>(t127: F, t371: F, t482: F, t481: F, t369: F, t479: F, t475: F, t467: F, t403: F, t414: F, t66: F, t1122: F, t247: F, t1221: F, t1222: F, t1227: F, t1231: F, t1235: F, t1238: F, t1247: F, t1252: F, t484: F, t225: F, t494: F, t460: F, t487: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t1256 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk424::<F>(t127, t371, t482);
        let (t1258, t1260) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk425::<F>(t1256, t481, t369, t479, t475);
        let t1261 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk426::<F>(t1260, t467);
        let t1263 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk427::<F>(t403, t414);
        let t1264 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk428::<F>(t1263, t66);
        let t1266 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk429::<F>(t1122, t1264, t247);
        let t1269 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk430::<F>(t1221, t1222, t1227, t1231, t1235, t1238, t1247, t1252, t1258, t1261, t1266, t484);
        let (t1271, t1274) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk431::<F>(t1269, t225, t494, t460, t487);
    (t1256, t1258, t1260, t1261, t1263, t1264, t1266, t1269, t1271, t1274)
}
