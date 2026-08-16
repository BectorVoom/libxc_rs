//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta77 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk458;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk459;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk460;
use chunk3::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk461;
use chunk4::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk462;
use chunk5::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk463;
use chunk6::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk464;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta77(t1544: f64, t832: f64, t1553: f64, t227: f64, t229: f64, t231: f64, t828: f64, t827: f64, t855: f64, t1549: f64, t797: f64, t799: f64, t812: f64, t819: f64, t825: f64, t848: f64, t851: f64, t225: f64, t257: f64, t879: f64, t234: f64, t213: f64, t820: f64, t873: f64, t878: f64, t868: f64, t783: f64, t791: f64, t865: f64, t1524: f64, t1533: f64, t1536: f64, t198: f64, t207: f64, t679: f64, t704: f64, t751: f64, t759: f64, t764: f64, t765: f64, t892: f64, t1469: f64, t905: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1555, t1558) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk458(t1544, t832, t1553, t227, t229);
        let t1559 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk459(t1558, t231);
        let (t1561, t1565, t1568) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk460(t1559, t828, t827, t1544, t855, t1549, t797, t799, t812, t819, t825, t848, t851);
        let (t1569, t1570, t1579) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk461(t1568, t225, t257, t1559, t879, t234, t213, t820, t873, t878);
        let t1580 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk462(t1579, t868);
        let (t1583, t1587) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk463(t1570, t1580, t213, t783, t791, t865, t1524, t1533, t1536, t1544, t198, t207, t679, t704, t751, t759, t764, t765, t892);
        let t1592 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk464(t1469, t905);
    (t1555, t1558, t1559, t1561, t1565, t1568, t1569, t1579, t1580, t1583, t1587, t1592)
}
