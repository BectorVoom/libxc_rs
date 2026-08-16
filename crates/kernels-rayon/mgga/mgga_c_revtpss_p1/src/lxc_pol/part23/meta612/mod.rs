//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta612 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2278;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2279;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2280;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2281;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2282;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2283;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta612(t24375: f64, t3523: f64, t16706: f64, t16876: f64, t20276: f64, t20278: f64, t20280: f64, t20283: f64, t20285: f64, t20287: f64, t24230: f64, t24234: f64, t24265: f64, t24267: f64, t24272: f64, t24275: f64, t12542: f64, t12543: f64, t24238: f64, t24242: f64, t24246: f64, t24250: f64, t24289: f64, t24292: f64, t24295: f64, t24298: f64, t24313: f64, t24315: f64, t24318: f64, t24320: f64, t1188: f64, t12555: f64, t1756: f64, t20671: f64, t1745: f64, t6502: f64, t1744: f64, t20618: f64, t1757: f64, t6534: f64, t1161: f64, t1180: f64, t12429: f64, t12470: f64, t12486: f64, t12553: f64, t17097: f64, t20526: f64, t20542: f64, t24214: f64, t24217: f64, t24331: f64, t24363: f64, t24366: f64, t3452: f64, t3477: f64, t3496: f64, t3521: f64, t5158: f64, t6535: f64, t6538: f64, t1169: f64, t24330: f64, t12397: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t24376, t24393) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2278(t24375, t3523, t16706, t16876, t20276, t20278, t20280, t20283, t20285, t20287, t24230, t24234, t24265, t24267, t24272, t24275);
        let t24406 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2279(t12542, t12543, t24238, t24242, t24246, t24250, t24289, t24292, t24295, t24298, t24313, t24315, t24318, t24320);
        let t24407 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2280(t24393, t24406);
        let (t24408, t24411, t24414, t24417, t24420, t24423) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2281(t1188, t24407, t12555, t24375, t1756, t20671, t1745, t6502, t1744, t20618, t1757, t6534);
        let t24428 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2282(t1161, t1180, t12429, t12470, t12486, t12553, t17097, t1745, t1757, t20526, t20542, t24214, t24217, t24331, t24363, t24366, t24376, t24408, t24411, t24414, t24417, t24420, t24423, t3452, t3477, t3496, t3521, t5158, t6535, t6538);
        let (t24431, t24436, t24453) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2283(t1169, t24330, t1188, t24375, t12397, t16706, t20283, t20285, t20287, t24230, t24234, t24238, t24242, t24246, t24250);
    (t24376, t24407, t24408, t24411, t24414, t24417, t24420, t24423, t24428, t24431, t24436, t24453)
}
