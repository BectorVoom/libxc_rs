//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta65 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;
mod chunk10;
mod chunk11;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk478;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk479;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk480;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk481;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk482;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk483;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk484;
use chunk7::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk485;
use chunk8::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk486;
use chunk9::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk487;
use chunk10::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk488;
use chunk11::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk489;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta65<F: Float>(t355: F, t471: F, t1248: F, t487: F, t1269: F, t489: F, t1204: F, t1234: F, t1281: F, t1285: F, t460: F, t490: F, t1277: F, t1210: F, t1215: F, t1271: F, t1274: F, t495: F, t498: F, t265: F, t502: F, t1128: F, t1153: F, t1193: F, t1195: F, t1200: F, t198: F, t336: F, t895: F, t33: F, t1113: F, t504: F, t57: F, t606: F, t1111: F, dens_threshold: F, rho1: F, zeta_threshold: F, t116: F, t93: F, t649: F, t670: F, t22: F, t583: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t1287 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk478::<F>(t355, t471);
        let t1288 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk479::<F>(t1248, t1287, t487);
        let t1291 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk480::<F>(t1269, t489);
        let t1294 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk481::<F>(t1204, t1234, t1281, t1285, t1288, t1291, t460, t490);
        let t1295 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk482::<F>(t1277, t1294);
        let t1298 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk483::<F>(t1204, t1210, t1215, t1271, t1274, t1295, t460, t495);
        let t1300 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk484::<F>(t498);
        let t1304 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk485::<F>(t265, t502, t1128, t1153, t1193, t1195, t1200, t1298, t1300, t198, t336, t895);
        let t1310 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk486::<F>(t33, t1113, t1304, t265, t504, t57, t606, t895, t1111, dens_threshold, rho1, zeta_threshold);
        let t1312 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk487::<F>(t116, t93);
        let t1315 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk488::<F>(t1312, t649, t670);
        let t1317 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk489::<F>(t22, t583);
    (t1287, t1288, t1291, t1294, t1295, t1298, t1300, t1304, t1310, t1312, t1315, t1317)
}
