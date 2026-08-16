//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta369 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1255;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1256;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1257;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1258;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1259;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta369<F: Float>(t1161: F, t1180: F, t12429: F, t12470: F, t12486: F, t12553: F, t17097: F, t1745: F, t1757: F, t20526: F, t20542: F, t24214: F, t24217: F, t24331: F, t24363: F, t24366: F, t24376: F, t24408: F, t24411: F, t24414: F, t24417: F, t24420: F, t24423: F, t3452: F, t3477: F, t3496: F, t3521: F, t5158: F, t6535: F, t6538: F, t1169: F, t24330: F, t1188: F, t24375: F, t12397: F, t16706: F, t20283: F, t20285: F, t20287: F, t24230: F, t24234: F, t24238: F, t24242: F, t24246: F, t24250: F, t12382: F, t422: F, t17023: F, t17032: F, t17154: F, t24219: F, t24223: F, t24253: F, t24257: F, t24259: F, t24261: F, t24264: F, t24326: F, t24329: F, t435: F, t5120: F, t6487: F, t6503: F, t6506: F, t6519: F, t300: F, t20895: F, t5184: F, t1196: F, t24255: F) -> (F, F, F, F, F, F, F, F, F) {
        let t24428 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1255::<F>(t1161, t1180, t12429, t12470, t12486, t12553, t17097, t1745, t1757, t20526, t20542, t24214, t24217, t24331, t24363, t24366, t24376, t24408, t24411, t24414, t24417, t24420, t24423, t3452, t3477, t3496, t3521, t5158, t6535, t6538);
        let (t24431, t24436, t24453) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1256::<F>(t1169, t24330, t1188, t24375, t12397, t16706, t20283, t20285, t20287, t24230, t24234, t24238, t24242, t24246, t24250);
        let (t24466, t24468) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1257::<F>(t12382, t16706, t20283, t20285, t20287, t24230, t24234, t24238, t24242, t24246, t24250, t422);
        let t24470 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1258::<F>(t17023, t17032, t17154, t24219, t24223, t24253, t24257, t24259, t24261, t24264, t24326, t24329, t24431, t24436, t24453, t24468, t3477, t3521, t435, t5120, t6487, t6503, t6506, t6519);
        let (t24472, t24473, t24475, t24476) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1259::<F>(t24428, t24470, t300, t20895, t5184, t1196, t24214, t24217, t24219, t24223, t24255, t24257, t24259, t24261, t24264, t24326, t24329);
    (t24431, t24436, t24453, t24466, t24468, t24472, t24473, t24475, t24476)
}
