//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta51 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk341;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk342;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk343;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk344;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk345;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk346;
use chunk6::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk347;
use chunk7::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk348;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta51<F: Float>(t1038: F, t479: F, t1244: F, t1241: F, t471: F, t73: F, t127: F, t371: F, t482: F, t481: F, t369: F, t475: F, t467: F, t403: F, t414: F, t66: F, t460: F, t487: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t1246, t1247) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk341::<F>(t1038, t479, t1244, t1241);
        let t1250 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk342::<F>(t471, t73);
        let t1256 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk343::<F>(t127, t371, t482);
        let (t1258, t1260) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk344::<F>(t1256, t481, t369, t479, t475);
        let t1261 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk345::<F>(t1260, t467);
        let t1263 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk346::<F>(t403, t414);
        let t1264 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk347::<F>(t1263, t66);
        let t1274 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk348::<F>(t460, t487);
    (t1246, t1247, t1250, t1256, t1258, t1260, t1261, t1263, t1264, t1274)
}
