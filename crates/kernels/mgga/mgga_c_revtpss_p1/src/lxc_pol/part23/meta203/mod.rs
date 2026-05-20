//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta203 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1207;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1208;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1209;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1210;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1211;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1212;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta203<F: Float>(t4101: F, t5741: F, t225: F, t3999: F, t213: F, t4086: F, t1892: F, t545: F, t869: F, t689: F, t72: F, t1432: F, t686: F, t1385: F, t1399: F, t1437: F, t1883: F, t4082: F, t4085: F, t4090: F, t4094: F, t4099: F, t4105: F, t4109: F, t4113: F, t4118: F, t546: F, t5659: F, t5675: F, t5710: F, t5735: F, t5738: F, t820: F, t1427: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t5742, t5744) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1207::<F>(t4101, t5741, t225, t3999);
        let t5745 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1208::<F>(t213, t5744);
        let t5755 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1209::<F>(t213, t4086);
        let (t5759, t5760, t5761, t5763, t5765, t5767) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1210::<F>(t1892, t545, t869, t689, t72, t1432, t686, t1385);
        let t5774 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1211::<F>(t1399, t1437, t1883, t213, t4082, t4085, t4090, t4094, t4099, t4105, t4109, t4113, t4118, t546, t5659, t5675, t5710, t5735, t5738, t5742, t5745, t5755, t5761, t5765, t5767, t820);
        let t5775 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1212::<F>(t1427, t5774);
    (t5742, t5744, t5745, t5755, t5759, t5760, t5761, t5763, t5765, t5767, t5774, t5775)
}
