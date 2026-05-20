//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta68 (260520-c91 hierarchical CSE).
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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk439;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk440;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk441;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk442;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk443;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk444;
use chunk6::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk445;
use chunk7::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk446;
use chunk8::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk447;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta68<F: Float>(t1204: F, t1210: F, t1215: F, t1271: F, t1274: F, t1295: F, t460: F, t495: F, t498: F, t33: F, t265: F, t502: F, t1128: F, t1153: F, t1193: F, t1195: F, t1200: F, t198: F, t336: F, t895: F, t1113: F, t504: F, t57: F, t606: F, dens_threshold: F, rho1: F, zeta_threshold: F, t1111: F, t116: F, t93: F, t649: F, t670: F, t22: F, t583: F, t521: F, t19: F, t588: F, t30: F, t513: F, t605: F, t516: F, t162: F, t189: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1298, t1300) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk439::<F>(t1204, t1210, t1215, t1271, t1274, t1295, t460, t495, t498);
        let (t1304, t1309) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk440::<F>(t33, t265, t502, t1128, t1153, t1193, t1195, t1200, t1298, t1300, t198, t336, t895, t1113, t504, t57, t606, dens_threshold, rho1, zeta_threshold);
        let t1310 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk441::<F>(t1111, t1309);
        let t1312 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk442::<F>(t116, t93);
        let t1315 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk443::<F>(t1312, t649, t670);
        let t1317 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk444::<F>(t22, t583);
        let (t1319, t1320) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk445::<F>(t1317, t521, t19, t588);
        let (t1322, t1330) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk446::<F>(t30, t33, t1320, t521, t513, t605, t1113, t516, t162, zeta_threshold);
        let t1331 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk447::<F>(t1330, t189);
    (t1298, t1300, t1304, t1310, t1312, t1315, t1317, t1319, t1320, t1322, t1330, t1331)
}
