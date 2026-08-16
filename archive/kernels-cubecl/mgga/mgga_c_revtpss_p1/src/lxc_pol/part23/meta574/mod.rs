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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta574<F: Float>(t1544: F, t18268: F, t18850: F, t198: F, t23106: F, t23110: F, t23111: F, t23114: F, t23123: F, t23124: F, t23127: F, t23128: F, t23129: F, t23130: F, t23148: F, t2403: F, t262: F, t4541: F, t765: F, t9394: F, t2723: F, t6016: F, t1558: F, t5977: F, t10871: F, t231: F, t10552: F, t10554: F, t23096: F, t23097: F, t23102: F, t23103: F, t9278: F, t9308: F, t9316: F, t9329: F, t9333: F, t18556: F, t10566: F, t23104: F, t18563: F, t4311: F, t5999: F, t10568: F, t10577: F, t10582: F, t10584: F, t10586: F, t9514: F, t9517: F, t9521: F, t9524: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t23152 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2173::<F>(t1544, t18268, t18850, t198, t23106, t23110, t23111, t23114, t23123, t23124, t23127, t23128, t23129, t23130, t23148, t2403, t262, t4541, t765, t9394);
        let t23160 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2174::<F>(t2723, t6016);
        let t23167 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2175::<F>(t1558, t5977);
        let t23168 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2176::<F>(t10871, t23167);
        let t23172 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2177::<F>(t23167, t2723);
        let t23177 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2178::<F>(t231, t23167);
        let (t23185, t23186) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2179::<F>(t10552, t10554, t23096, t23097, t23102, t23103, t9278, t9308, t9316, t9329, t9333, t18556);
        let (t23187, t23189) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2180::<F>(t10566, t23104, t23106, t23110, t23123, t23127, t23128, t23129, t23130, t23186, t9394, t18563);
        let (t23191, t23192) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2181::<F>(t4311, t5999, t10568, t10577, t10582, t10584, t10586, t23189, t9514, t9517, t9521, t9524);
    (t23152, t23160, t23167, t23168, t23172, t23177, t23185, t23186, t23187, t23189, t23191, t23192)
}
