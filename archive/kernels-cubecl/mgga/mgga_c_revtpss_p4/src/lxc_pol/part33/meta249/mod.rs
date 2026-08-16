//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta249 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1103;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1104;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1105;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1106;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1107;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta249<F: Float>(t1832: F, t1300: F, t198: F, t336: F, t3801: F, t6435: F, t6437: F, t6441: F, t6473: F, t6476: F, t6542: F, t6544: F, t6546: F, t6550: F, t6554: F, t6558: F, t6748: F, t33: F, t265: F, t502: F, t6084: F, t1469: F, t1587: F, t1711: F, t1837: F, t504: F, t57: F, t5825: F, t6416: F, dens_threshold: F, rho1: F, zeta_threshold: F, t6412: F, t1312: F, t1518: F, t4248: F, t5877: F, t5883: F, t5920: F, t93: F, t5545: F, t5547: F, t5570: F, t5572: F, t1907: F, t30: F, t1468: F, t3833: F, t513: F, t5824: F, t3841: F, t516: F, t162: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t6752, t6756) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1103::<F>(t1832, t1300, t198, t336, t3801, t6435, t6437, t6441, t6473, t6476, t6542, t6544, t6546, t6550, t6554, t6558, t6748);
        let (t6757, t6764) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1104::<F>(t33, t265, t502, t6084, t6756, t1469, t1587, t1711, t1837, t504, t57, t5825, t6416, dens_threshold, rho1, zeta_threshold);
        let t6765 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1105::<F>(t6412, t6764);
        let (t6773, t6777, t6778, t6779, t6780, t6781) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1106::<F>(t1312, t1518, t4248, t5877, t5883, t5920, t93, t5545, t5547, t5570, t5572, t1907);
        let (t6785, t6792, t6800) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1107::<F>(t30, t33, t1468, t3833, t513, t5824, t1711, t3841, t516, t6416, t162, zeta_threshold);
    (t6752, t6757, t6765, t6773, t6777, t6778, t6779, t6780, t6781, t6785, t6792, t6800)
}
