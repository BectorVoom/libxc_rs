//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta74 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk445;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk446;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk447;
use chunk3::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk448;
use chunk4::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk449;
use chunk5::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk450;
use chunk6::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk451;
use chunk7::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk452;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta74(t555: f64, t72: f64, t1432: f64, t686: f64, t1385: f64, t1399: f64, t1419: f64, t546: f64, t1431: f64, t213: f64, t820: f64, t1427: f64, t1361: f64, t1366: f64, t1421: f64, t1424: f64, t565: f64, t1319: f64, t1322: f64, t1332: f64, t1334: f64, t1336: f64, t1339: f64, t1342: f64, t1343: f64, t1353: f64, t198: f64, t532: f64, t679: f64, t704: f64, t118: f64, t1310: f64, t1315: f64, t508: f64, t511: f64, t569: f64, t649: f64, t651: f64, t671: f64, t3: f64, t571: f64, param_d: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1433, t1436, t1437) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk445(t555, t72, t1432, t686, t1385);
        let t1444 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk446(t1399, t1437, t1419, t546, t1431, t1436, t213, t820);
        let t1445 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk447(t1427, t1444);
        let t1448 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk448(t1361, t1366, t1421, t1424, t1445, t213);
        let t1450 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk449(t565);
        let t1453 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk450(t1319, t1322, t1332, t1334, t1336, t1339, t1342, t1343, t1353, t1448, t1450, t198, t532, t679, t704);
        let (t1455, t1456, t1458) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk451(t118, t1310, t1315, t1453, t508, t511, t569, t649, t651, t671, t3, t571);
        let t1459 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk452(t1455, param_d);
    (t1433, t1436, t1437, t1444, t1445, t1448, t1450, t1453, t1455, t1456, t1458, t1459)
}
