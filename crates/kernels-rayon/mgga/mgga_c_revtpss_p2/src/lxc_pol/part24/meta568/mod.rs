//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta568 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1740;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1741;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1742;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1743;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta568(t68255: f64, t81156: f64, t81158: f64, t89824: f64, t89828: f64, t89832: f64, t89839: f64, t89843: f64, t89847: f64, t89851: f64, t89855: f64, t44865: f64, t56236: f64, t68257: f64, t68399: f64, t81230: f64, t81232: f64, t81234: f64, t81236: f64, t89865: f64, t89869: f64, t89873: f64, t89877: f64, t6573: f64, t6628: f64, t1774: f64, t22688: f64, t1042: f64, t1261: f64, t17202: f64, t17569: f64, t24612: f64, t24773: f64, t3711: f64, t5268: f64, t5293: f64, t5819: f64, t6587: f64, t69668: f64, t69700: f64, t82338: f64, t82351: f64, t82434: f64, t82441: f64, t88732: f64, t1794: f64, t24633: f64, t6622: f64, t1250: f64, t12809: f64, t12910: f64, t1797: f64, t1808: f64, t24741: f64, t3611: f64, t3718: f64, t3720: f64, t5302: f64, t5384: f64, t57147: f64, t82469: f64, t82491: f64, t82534: f64, t82536: f64, t83296: f64, t83728: f64, t482: f64, t21275: f64, t22671: f64, t24605: f64, t24649: f64, t24726: f64, t24836: f64, t5296: f64, t5381: f64, t59162: f64, t6635: f64, t70319: f64, t82595: f64, t82603: f64, t82656: f64, t82678: f64, t88916: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t89947, t89959) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1740(t68255, t81156, t81158, t89824, t89828, t89832, t89839, t89843, t89847, t89851, t89855, t44865, t56236, t68257, t68399, t81230, t81232, t81234, t81236, t89865, t89869, t89873, t89877);
        let (t89960, t89978, t90001, t90012) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1741(t89947, t89959, t6573, t6628, t1774, t22688, t1042, t1261, t17202, t17569, t24612, t24773, t3711, t5268, t5293, t5819, t6587, t69668, t69700, t82338, t82351, t82434, t82441, t88732);
        let (t90037, t90042, t90054, t90059, t90066) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1742(t5819, t6573, t6587, t6628, t1794, t24633, t6622, t1042, t1250, t12809, t12910, t1797, t1808, t24741, t3611, t3718, t3720, t5302, t5384, t57147, t82469, t82491, t82534, t82536, t83296, t83728);
        let (t90080, t90081, t90116) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1743(t6622, t482, t1042, t1261, t17569, t1774, t21275, t22671, t24605, t24649, t24726, t24836, t3711, t5296, t5302, t5381, t59162, t6635, t70319, t82595, t82603, t82656, t82678, t88916);
    (t89960, t89978, t90001, t90012, t90037, t90042, t90054, t90059, t90066, t90080, t90081, t90116)
}
