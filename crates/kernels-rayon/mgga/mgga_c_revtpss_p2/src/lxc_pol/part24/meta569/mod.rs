//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta569 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1744;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1745;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1746;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1747;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta569(t6628: f64, t482: f64, t1774: f64, t24543: f64, t1794: f64, t24616: f64, t17687: f64, t5819: f64, t1042: f64, t1250: f64, t1261: f64, t12787: f64, t13063: f64, t17448: f64, t17569: f64, t21040: f64, t24535: f64, t24546: f64, t247: f64, t24759: f64, t24787: f64, t3618: f64, t3625: f64, t3720: f64, t44375: f64, t44378: f64, t44448: f64, t44449: f64, t44609: f64, t45371: f64, t5391: f64, t56731: f64, t82749: f64, t89837: f64, t17505: f64, t1797: f64, t21107: f64, t24612: f64, t3610: f64, t3611: f64, t5268: f64, t5296: f64, t5384: f64, t5825: f64, t6573: f64, t6625: f64, t6631: f64, t6635: f64, t71693: f64, t71699: f64, t82555: f64, t82821: f64, t82824: f64, t82827: f64, t90037: f64, t90081: f64, t1469: f64, t17643: f64, t24494: f64, t5192: f64, t68255: f64, t81156: f64, t81158: f64, t89824: f64, t89828: f64, t89832: f64, t89839: f64, t89843: f64, t89847: f64, t89851: f64, t89855: f64, t43995: f64, t56236: f64, t68257: f64, t68399: f64, t81230: f64, t81232: f64, t81234: f64, t81236: f64, t89865: f64, t89869: f64, t89873: f64, t89877: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t90132, t90133, t90162, t90167, t90180, t90185) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1744(t6628, t482, t1774, t24543, t1794, t24616, t17687, t5819, t1042, t1250, t1261, t12787, t13063, t17448, t17569, t21040, t24535, t24546, t247, t24759, t24787, t3618, t3625, t3720, t44375, t44378, t44448, t44449, t44609, t45371, t5391, t56731, t82749, t89837);
        let t90245 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1745(t1042, t17505, t1797, t21107, t24612, t3610, t3611, t5268, t5296, t5384, t5825, t6573, t6625, t6631, t6635, t71693, t71699, t82555, t82821, t82824, t82827, t90037, t90081);
        let (t90253, t90262, t90293, t90305) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1746(t1469, t1774, t17643, t5819, t24494, t5192, t68255, t81156, t81158, t89824, t89828, t89832, t89839, t89843, t89847, t89851, t89855);
        let t90317 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1747(t43995, t56236, t68257, t68399, t81230, t81232, t81234, t81236, t89865, t89869, t89873, t89877);
    (t90132, t90133, t90162, t90167, t90180, t90185, t90245, t90253, t90262, t90293, t90305, t90317)
}
