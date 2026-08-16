//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta369 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1255;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1256;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1257;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1258;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1259;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta369(t1161: f64, t1180: f64, t12429: f64, t12470: f64, t12486: f64, t12553: f64, t17097: f64, t1745: f64, t1757: f64, t20526: f64, t20542: f64, t24214: f64, t24217: f64, t24331: f64, t24363: f64, t24366: f64, t24376: f64, t24408: f64, t24411: f64, t24414: f64, t24417: f64, t24420: f64, t24423: f64, t3452: f64, t3477: f64, t3496: f64, t3521: f64, t5158: f64, t6535: f64, t6538: f64, t1169: f64, t24330: f64, t1188: f64, t24375: f64, t12397: f64, t16706: f64, t20283: f64, t20285: f64, t20287: f64, t24230: f64, t24234: f64, t24238: f64, t24242: f64, t24246: f64, t24250: f64, t12382: f64, t422: f64, t17023: f64, t17032: f64, t17154: f64, t24219: f64, t24223: f64, t24253: f64, t24257: f64, t24259: f64, t24261: f64, t24264: f64, t24326: f64, t24329: f64, t435: f64, t5120: f64, t6487: f64, t6503: f64, t6506: f64, t6519: f64, t300: f64, t20895: f64, t5184: f64, t1196: f64, t24255: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t24428 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1255(t1161, t1180, t12429, t12470, t12486, t12553, t17097, t1745, t1757, t20526, t20542, t24214, t24217, t24331, t24363, t24366, t24376, t24408, t24411, t24414, t24417, t24420, t24423, t3452, t3477, t3496, t3521, t5158, t6535, t6538);
        let (t24431, t24436, t24453) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1256(t1169, t24330, t1188, t24375, t12397, t16706, t20283, t20285, t20287, t24230, t24234, t24238, t24242, t24246, t24250);
        let (t24466, t24468) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1257(t12382, t16706, t20283, t20285, t20287, t24230, t24234, t24238, t24242, t24246, t24250, t422);
        let t24470 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1258(t17023, t17032, t17154, t24219, t24223, t24253, t24257, t24259, t24261, t24264, t24326, t24329, t24431, t24436, t24453, t24468, t3477, t3521, t435, t5120, t6487, t6503, t6506, t6519);
        let (t24472, t24473, t24475, t24476) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1259(t24428, t24470, t300, t20895, t5184, t1196, t24214, t24217, t24219, t24223, t24255, t24257, t24259, t24261, t24264, t24326, t24329);
    (t24431, t24436, t24453, t24466, t24468, t24472, t24473, t24475, t24476)
}
