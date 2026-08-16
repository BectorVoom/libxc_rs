//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta469 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2157;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2158;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2159;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2160;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta469(t15670: f64, t366: f64, t3106: f64, t4817: f64, t1025: f64, t1028: f64, t11644: f64, t11649: f64, t11783: f64, t15651: f64, t15656: f64, t15662: f64, t15668: f64, t1665: f64, t3208: f64, t3211: f64, t3220: f64, t3224: f64, t4854: f64, t4858: f64, t11710: f64, t4787: f64, t3091: f64, t245: f64, t4890: f64, t3088: f64, t3317: f64, t1065: f64, t1668: f64, t372: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15671, t15675, t15676) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2157(t15670, t366, t3106, t4817, t1025, t1028, t11644, t11649, t11783, t15651, t15656, t15662, t15668, t1665, t3208, t3211, t3220, t3224, t4854, t4858);
        let (t15682, t15684, t15687, t15688) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2158(t11710, t4787, t3091, t245, t4890, t3088);
        let t15689 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2159(t15688, t3317);
        let (t15690, t15691) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2160(t1065, t1668, t372);
    (t15671, t15675, t15676, t15682, t15684, t15687, t15688, t15689, t15690, t15691)
}
