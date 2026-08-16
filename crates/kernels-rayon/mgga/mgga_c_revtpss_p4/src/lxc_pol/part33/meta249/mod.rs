//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta249 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1103;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1104;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1105;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1106;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1107;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta249(t1832: f64, t1300: f64, t198: f64, t336: f64, t3801: f64, t6435: f64, t6437: f64, t6441: f64, t6473: f64, t6476: f64, t6542: f64, t6544: f64, t6546: f64, t6550: f64, t6554: f64, t6558: f64, t6748: f64, t33: f64, t265: f64, t502: f64, t6084: f64, t1469: f64, t1587: f64, t1711: f64, t1837: f64, t504: f64, t57: f64, t5825: f64, t6416: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64, t6412: f64, t1312: f64, t1518: f64, t4248: f64, t5877: f64, t5883: f64, t5920: f64, t93: f64, t5545: f64, t5547: f64, t5570: f64, t5572: f64, t1907: f64, t30: f64, t1468: f64, t3833: f64, t513: f64, t5824: f64, t3841: f64, t516: f64, t162: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6752, t6756) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1103(t1832, t1300, t198, t336, t3801, t6435, t6437, t6441, t6473, t6476, t6542, t6544, t6546, t6550, t6554, t6558, t6748);
        let (t6757, t6764) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1104(t33, t265, t502, t6084, t6756, t1469, t1587, t1711, t1837, t504, t57, t5825, t6416, dens_threshold, rho1, zeta_threshold);
        let t6765 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1105(t6412, t6764);
        let (t6773, t6777, t6778, t6779, t6780, t6781) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1106(t1312, t1518, t4248, t5877, t5883, t5920, t93, t5545, t5547, t5570, t5572, t1907);
        let (t6785, t6792, t6800) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1107(t30, t33, t1468, t3833, t513, t5824, t1711, t3841, t516, t6416, t162, zeta_threshold);
    (t6752, t6757, t6765, t6773, t6777, t6778, t6779, t6780, t6781, t6785, t6792, t6800)
}
