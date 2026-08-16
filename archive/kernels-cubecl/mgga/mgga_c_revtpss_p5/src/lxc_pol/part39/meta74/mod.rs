//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta74 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk445;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk446;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk447;
use chunk3::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk448;
use chunk4::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk449;
use chunk5::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk450;
use chunk6::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk451;
use chunk7::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk452;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta74<F: Float>(t555: F, t72: F, t1432: F, t686: F, t1385: F, t1399: F, t1419: F, t546: F, t1431: F, t213: F, t820: F, t1427: F, t1361: F, t1366: F, t1421: F, t1424: F, t565: F, t1319: F, t1322: F, t1332: F, t1334: F, t1336: F, t1339: F, t1342: F, t1343: F, t1353: F, t198: F, t532: F, t679: F, t704: F, t118: F, t1310: F, t1315: F, t508: F, t511: F, t569: F, t649: F, t651: F, t671: F, t3: F, t571: F, param_d: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1433, t1436, t1437) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk445::<F>(t555, t72, t1432, t686, t1385);
        let t1444 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk446::<F>(t1399, t1437, t1419, t546, t1431, t1436, t213, t820);
        let t1445 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk447::<F>(t1427, t1444);
        let t1448 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk448::<F>(t1361, t1366, t1421, t1424, t1445, t213);
        let t1450 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk449::<F>(t565);
        let t1453 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk450::<F>(t1319, t1322, t1332, t1334, t1336, t1339, t1342, t1343, t1353, t1448, t1450, t198, t532, t679, t704);
        let (t1455, t1456, t1458) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk451::<F>(t118, t1310, t1315, t1453, t508, t511, t569, t649, t651, t671, t3, t571);
        let t1459 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk452::<F>(t1455, param_d);
    (t1433, t1436, t1437, t1444, t1445, t1448, t1450, t1453, t1455, t1456, t1458, t1459)
}
