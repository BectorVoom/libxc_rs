//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta568 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1740;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1741;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1742;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1743;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta568<F: Float>(t68255: F, t81156: F, t81158: F, t89824: F, t89828: F, t89832: F, t89839: F, t89843: F, t89847: F, t89851: F, t89855: F, t44865: F, t56236: F, t68257: F, t68399: F, t81230: F, t81232: F, t81234: F, t81236: F, t89865: F, t89869: F, t89873: F, t89877: F, t6573: F, t6628: F, t1774: F, t22688: F, t1042: F, t1261: F, t17202: F, t17569: F, t24612: F, t24773: F, t3711: F, t5268: F, t5293: F, t5819: F, t6587: F, t69668: F, t69700: F, t82338: F, t82351: F, t82434: F, t82441: F, t88732: F, t1794: F, t24633: F, t6622: F, t1250: F, t12809: F, t12910: F, t1797: F, t1808: F, t24741: F, t3611: F, t3718: F, t3720: F, t5302: F, t5384: F, t57147: F, t82469: F, t82491: F, t82534: F, t82536: F, t83296: F, t83728: F, t482: F, t21275: F, t22671: F, t24605: F, t24649: F, t24726: F, t24836: F, t5296: F, t5381: F, t59162: F, t6635: F, t70319: F, t82595: F, t82603: F, t82656: F, t82678: F, t88916: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t89947, t89959) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1740::<F>(t68255, t81156, t81158, t89824, t89828, t89832, t89839, t89843, t89847, t89851, t89855, t44865, t56236, t68257, t68399, t81230, t81232, t81234, t81236, t89865, t89869, t89873, t89877);
        let (t89960, t89978, t90001, t90012) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1741::<F>(t89947, t89959, t6573, t6628, t1774, t22688, t1042, t1261, t17202, t17569, t24612, t24773, t3711, t5268, t5293, t5819, t6587, t69668, t69700, t82338, t82351, t82434, t82441, t88732);
        let (t90037, t90042, t90054, t90059, t90066) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1742::<F>(t5819, t6573, t6587, t6628, t1794, t24633, t6622, t1042, t1250, t12809, t12910, t1797, t1808, t24741, t3611, t3718, t3720, t5302, t5384, t57147, t82469, t82491, t82534, t82536, t83296, t83728);
        let (t90080, t90081, t90116) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1743::<F>(t6622, t482, t1042, t1261, t17569, t1774, t21275, t22671, t24605, t24649, t24726, t24836, t3711, t5296, t5302, t5381, t59162, t6635, t70319, t82595, t82603, t82656, t82678, t88916);
    (t89960, t89978, t90001, t90012, t90037, t90042, t90054, t90059, t90066, t90080, t90081, t90116)
}
