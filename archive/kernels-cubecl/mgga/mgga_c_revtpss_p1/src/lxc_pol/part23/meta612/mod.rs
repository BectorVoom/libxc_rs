//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta612 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2278;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2279;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2280;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2281;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2282;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2283;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta612<F: Float>(t24375: F, t3523: F, t16706: F, t16876: F, t20276: F, t20278: F, t20280: F, t20283: F, t20285: F, t20287: F, t24230: F, t24234: F, t24265: F, t24267: F, t24272: F, t24275: F, t12542: F, t12543: F, t24238: F, t24242: F, t24246: F, t24250: F, t24289: F, t24292: F, t24295: F, t24298: F, t24313: F, t24315: F, t24318: F, t24320: F, t1188: F, t12555: F, t1756: F, t20671: F, t1745: F, t6502: F, t1744: F, t20618: F, t1757: F, t6534: F, t1161: F, t1180: F, t12429: F, t12470: F, t12486: F, t12553: F, t17097: F, t20526: F, t20542: F, t24214: F, t24217: F, t24331: F, t24363: F, t24366: F, t3452: F, t3477: F, t3496: F, t3521: F, t5158: F, t6535: F, t6538: F, t1169: F, t24330: F, t12397: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t24376, t24393) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2278::<F>(t24375, t3523, t16706, t16876, t20276, t20278, t20280, t20283, t20285, t20287, t24230, t24234, t24265, t24267, t24272, t24275);
        let t24406 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2279::<F>(t12542, t12543, t24238, t24242, t24246, t24250, t24289, t24292, t24295, t24298, t24313, t24315, t24318, t24320);
        let t24407 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2280::<F>(t24393, t24406);
        let (t24408, t24411, t24414, t24417, t24420, t24423) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2281::<F>(t1188, t24407, t12555, t24375, t1756, t20671, t1745, t6502, t1744, t20618, t1757, t6534);
        let t24428 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2282::<F>(t1161, t1180, t12429, t12470, t12486, t12553, t17097, t1745, t1757, t20526, t20542, t24214, t24217, t24331, t24363, t24366, t24376, t24408, t24411, t24414, t24417, t24420, t24423, t3452, t3477, t3496, t3521, t5158, t6535, t6538);
        let (t24431, t24436, t24453) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2283::<F>(t1169, t24330, t1188, t24375, t12397, t16706, t20283, t20285, t20287, t24230, t24234, t24238, t24242, t24246, t24250);
    (t24376, t24407, t24408, t24411, t24414, t24417, t24420, t24423, t24428, t24431, t24436, t24453)
}
