//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta569 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1744;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1745;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1746;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1747;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta569<F: Float>(t6628: F, t482: F, t1774: F, t24543: F, t1794: F, t24616: F, t17687: F, t5819: F, t1042: F, t1250: F, t1261: F, t12787: F, t13063: F, t17448: F, t17569: F, t21040: F, t24535: F, t24546: F, t247: F, t24759: F, t24787: F, t3618: F, t3625: F, t3720: F, t44375: F, t44378: F, t44448: F, t44449: F, t44609: F, t45371: F, t5391: F, t56731: F, t82749: F, t89837: F, t17505: F, t1797: F, t21107: F, t24612: F, t3610: F, t3611: F, t5268: F, t5296: F, t5384: F, t5825: F, t6573: F, t6625: F, t6631: F, t6635: F, t71693: F, t71699: F, t82555: F, t82821: F, t82824: F, t82827: F, t90037: F, t90081: F, t1469: F, t17643: F, t24494: F, t5192: F, t68255: F, t81156: F, t81158: F, t89824: F, t89828: F, t89832: F, t89839: F, t89843: F, t89847: F, t89851: F, t89855: F, t43995: F, t56236: F, t68257: F, t68399: F, t81230: F, t81232: F, t81234: F, t81236: F, t89865: F, t89869: F, t89873: F, t89877: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t90132, t90133, t90162, t90167, t90180, t90185) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1744::<F>(t6628, t482, t1774, t24543, t1794, t24616, t17687, t5819, t1042, t1250, t1261, t12787, t13063, t17448, t17569, t21040, t24535, t24546, t247, t24759, t24787, t3618, t3625, t3720, t44375, t44378, t44448, t44449, t44609, t45371, t5391, t56731, t82749, t89837);
        let t90245 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1745::<F>(t1042, t17505, t1797, t21107, t24612, t3610, t3611, t5268, t5296, t5384, t5825, t6573, t6625, t6631, t6635, t71693, t71699, t82555, t82821, t82824, t82827, t90037, t90081);
        let (t90253, t90262, t90293, t90305) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1746::<F>(t1469, t1774, t17643, t5819, t24494, t5192, t68255, t81156, t81158, t89824, t89828, t89832, t89839, t89843, t89847, t89851, t89855);
        let t90317 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1747::<F>(t43995, t56236, t68257, t68399, t81230, t81232, t81234, t81236, t89865, t89869, t89873, t89877);
    (t90132, t90133, t90162, t90167, t90180, t90185, t90245, t90253, t90262, t90293, t90305, t90317)
}
