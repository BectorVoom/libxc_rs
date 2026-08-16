//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta71 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk426;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk427;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk428;
use chunk3::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk429;
use chunk4::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk430;
use chunk5::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk431;
use chunk6::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk432;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta71(t124: f64, t1353: f64, t800: f64, t546: f64, t550: f64, t808: f64, t807: f64, t547: f64, t786: f64, t814: f64, t816: f64, t544: f64, t235: f64, t239: f64, t820: f64, t240: f64, t72: f64, t1319: f64, t1322: f64, t1332: f64, t1334: f64, t1336: f64, t1339: f64, t1342: f64, t225: f64, t679: f64, t704: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1372, t1376, t1378, t1379, t1383, t1384) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk426(t124, t1353, t800, t546, t550, t808, t807, t547, t786, t814, t816, t544);
        let t1385 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk427(t1384);
        let t1386 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk428(t1385, t235);
        let t1388 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk429(t1386, t239, t820);
        let t1389 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk430(t240, t550);
        let t1390 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk431(t1389, t72);
        let t1392 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk432(t1319, t1322, t1332, t1334, t1336, t1339, t1342, t225, t679, t704);
    (t1372, t1376, t1378, t1379, t1383, t1384, t1385, t1386, t1388, t1389, t1390, t1392)
}
