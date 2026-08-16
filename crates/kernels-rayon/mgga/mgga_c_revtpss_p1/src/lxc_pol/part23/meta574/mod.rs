//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta574 (260520-c91 hierarchical CSE).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2173;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2174;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2175;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2176;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2177;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2178;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2179;
use chunk7::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2180;
use chunk8::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2181;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta574(t1544: f64, t18268: f64, t18850: f64, t198: f64, t23106: f64, t23110: f64, t23111: f64, t23114: f64, t23123: f64, t23124: f64, t23127: f64, t23128: f64, t23129: f64, t23130: f64, t23148: f64, t2403: f64, t262: f64, t4541: f64, t765: f64, t9394: f64, t2723: f64, t6016: f64, t1558: f64, t5977: f64, t10871: f64, t231: f64, t10552: f64, t10554: f64, t23096: f64, t23097: f64, t23102: f64, t23103: f64, t9278: f64, t9308: f64, t9316: f64, t9329: f64, t9333: f64, t18556: f64, t10566: f64, t23104: f64, t18563: f64, t4311: f64, t5999: f64, t10568: f64, t10577: f64, t10582: f64, t10584: f64, t10586: f64, t9514: f64, t9517: f64, t9521: f64, t9524: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t23152 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2173(t1544, t18268, t18850, t198, t23106, t23110, t23111, t23114, t23123, t23124, t23127, t23128, t23129, t23130, t23148, t2403, t262, t4541, t765, t9394);
        let t23160 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2174(t2723, t6016);
        let t23167 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2175(t1558, t5977);
        let t23168 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2176(t10871, t23167);
        let t23172 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2177(t23167, t2723);
        let t23177 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2178(t231, t23167);
        let (t23185, t23186) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2179(t10552, t10554, t23096, t23097, t23102, t23103, t9278, t9308, t9316, t9329, t9333, t18556);
        let (t23187, t23189) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2180(t10566, t23104, t23106, t23110, t23123, t23127, t23128, t23129, t23130, t23186, t9394, t18563);
        let (t23191, t23192) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2181(t4311, t5999, t10568, t10577, t10582, t10584, t10586, t23189, t9514, t9517, t9521, t9524);
    (t23152, t23160, t23167, t23168, t23172, t23177, t23185, t23186, t23187, t23189, t23191, t23192)
}
