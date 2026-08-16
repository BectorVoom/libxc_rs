//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta203 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1207;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1208;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1209;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1210;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1211;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1212;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta203(t4101: f64, t5741: f64, t225: f64, t3999: f64, t213: f64, t4086: f64, t1892: f64, t545: f64, t869: f64, t689: f64, t72: f64, t1432: f64, t686: f64, t1385: f64, t1399: f64, t1437: f64, t1883: f64, t4082: f64, t4085: f64, t4090: f64, t4094: f64, t4099: f64, t4105: f64, t4109: f64, t4113: f64, t4118: f64, t546: f64, t5659: f64, t5675: f64, t5710: f64, t5735: f64, t5738: f64, t820: f64, t1427: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5742, t5744) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1207(t4101, t5741, t225, t3999);
        let t5745 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1208(t213, t5744);
        let t5755 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1209(t213, t4086);
        let (t5759, t5760, t5761, t5763, t5765, t5767) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1210(t1892, t545, t869, t689, t72, t1432, t686, t1385);
        let t5774 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1211(t1399, t1437, t1883, t213, t4082, t4085, t4090, t4094, t4099, t4105, t4109, t4113, t4118, t546, t5659, t5675, t5710, t5735, t5738, t5742, t5745, t5755, t5761, t5765, t5767, t820);
        let t5775 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1212(t1427, t5774);
    (t5742, t5744, t5745, t5755, t5759, t5760, t5761, t5763, t5765, t5767, t5774, t5775)
}
