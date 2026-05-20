//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta61 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk407;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk408;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk409;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk410;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk411;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk412;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta61<F: Float>(t1243: F, t474: F, t1038: F, t479: F, t1241: F, t1128: F, t1153: F, t1193: F, t1195: F, t1200: F, t471: F, t73: F, t482: F, t1042: F, t127: F, t371: F, t481: F, t369: F, t475: F, t467: F, t403: F, t414: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1244, t1246, t1247) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk407::<F>(t1243, t474, t1038, t479, t1241);
        let t1248 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk408::<F>(t1128, t1153, t1193, t1195, t1200);
        let t1250 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk409::<F>(t471, t73);
        let (t1251, t1252, t1256, t1258, t1260) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk410::<F>(t1248, t1250, t482, t1042, t127, t371, t481, t369, t479, t475);
        let t1261 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk411::<F>(t1260, t467);
        let t1263 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk412::<F>(t403, t414);
    (t1244, t1246, t1247, t1248, t1250, t1251, t1252, t1256, t1258, t1260, t1261, t1263)
}
