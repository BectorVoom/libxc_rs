//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta550 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;
mod chunk10;
mod chunk11;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1626;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1627;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1628;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1629;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1630;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1631;
use chunk6::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1632;
use chunk7::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1633;
use chunk8::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1634;
use chunk9::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1635;
use chunk10::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1636;
use chunk11::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1637;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta550<F: Float>(t40325: F, t87399: F, t10871: F, t10698: F, t10870: F, t1544: F, t23148: F, t2477: F, t2721: F, t40324: F, t51170: F, t5962: F, t5966: F, t62251: F, t62399: F, t62401: F, t62431: F, t62443: F, t62445: F, t76878: F, t76882: F, t76887: F, t77127: F, t77131: F, t825: F, t827: F, t828: F, t851: F, t87395: F, t87400: F, t87470: F, t87503: F, t87562: F, t87579: F, t87608: F, t87721: F, t87742: F, t213: F, t234: F, t62633: F, t76117: F, t76125: F, t76134: F, t76139: F, t76144: F, t76153: F, t76158: F, t76163: F, t76172: F, t14546: F, t18677: F, t39649: F, t39652: F, t51390: F, t51403: F, t51408: F, t62684: F, t62716: F, t62723: F, t76237: F, t76242: F, t76255: F, t14961: F, t1559: F, t23172: F, t40314: F, t40316: F, t4514: F, t51553: F, t62843: F, t62847: F, t62874: F, t62907: F, t76127: F, t77191: F, t77197: F, t820: F, t14586: F, t4504: F, t51578: F, t51635: F, t6017: F, t62909: F, t62920: F, t62922: F, t62952: F, t62983: F, t62999: F, t77159: F, t77225: F, t10952: F, t18714: F, t23168: F, t23177: F, t40902: F, t4526: F, t51498: F, t51646: F, t51660: F, t51676: F, t51686: F, t5978: F, t87714: F, t87729: F, t879: F, t11008: F, t1579: F, t1580: F, t18699: F, t18800: F, t225: F, t23244: F, t23245: F, t23383: F, t23404: F, t257: F, t2723: F, t2770: F, t2811: F, t39633: F, t39697: F, t39723: F, t40294: F, t41095: F, t4474: F, t4494: F, t51213: F, t51237: F, t51246: F, t51445: F, t51452: F, t51733: F, t6022: F, t6048: F, t6071: F, t6072: F, t62649: F, t62651: F, t62653: F, t62665: F, t62670: F, t62777: F, t62929: F, t63050: F, t63058: F, t63099: F, t76081: F, t76100: F, t76104: F, t76108: F, t76182: F, t76206: F, t76223: F, t77171: F, t77177: F, t77183: F, t77316: F, t865: F, t868: F, t87629: F, t6075: F, t198: F, t207: F, t2403: F, t2411: F, t39537: F, t39540: F, t39741: F, t39744: F, t39747: F, t39750: F, t39756: F, t39760: F, t39764: F, t77460: F, t87318: F, t87342: F, t87357: F, t87373: F, t87640: F, t892: F, t1583: F, t18268: F, t23114: F, t2393: F, t39770: F, t39773: F, t39783: F, t39786: F, t39791: F, t39795: F, t4541: F, t87548: F, t87641: F, t87642: F, t87643: F, t87644: F, t87649: F, t39799: F, t39807: F, t39813: F, t39818: F, t39823: F, t40084: F, t40088: F, t40099: F, t40103: F, t77341: F, t87650: F, t87651: F, t39989: F, t40115: F, t40131: F, t40137: F, t87655: F, t87658: F, t87660: F, t87661: F, t87662: F, t87663: F, t87666: F, t87667: F, t87668: F, t87669: F, t18865: F, t1940: F, t23279: F, t29598: F, t40067: F, t40072: F, t40167: F, t40171: F, t40184: F, t4546: F, t6079: F, t61033: F, t77333: F, t87670: F, t87671: F, t87673: F, t87674: F, t87675: F) -> (F, F, F, F, F) {
        let (t87764, t87775, t87783) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1626::<F>(t40325, t87399, t10871, t10698, t10870, t1544, t23148, t2477, t2721, t40324, t51170, t5962, t5966, t62251, t62399, t62401, t62431, t62443, t62445, t76878, t76882, t76887, t77127, t77131, t825, t827, t828, t851, t87395, t87400);
        let (t87786, t87798) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1627::<F>(t87470, t87503, t87562, t87579, t87608, t87721, t87742, t87783, t213, t234, t62633, t76117, t76125, t76134, t76139, t76144, t76153, t76158, t76163, t76172);
        let t87824 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1628::<F>(t14546, t18677, t39649, t39652, t51390, t51403, t51408, t62684, t62716, t62723, t76237, t76242, t76255);
        let t87850 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1629::<F>(t14961, t1559, t23172, t40314, t40316, t4514, t51553, t62843, t62847, t62874, t62907, t76127, t77191, t77197, t820);
        let t87869 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1630::<F>(t14586, t1559, t18677, t4504, t4514, t51578, t51635, t6017, t62909, t62920, t62922, t62952, t62983, t62999, t77159, t77225, t820);
        let t87895 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1631::<F>(t10952, t18714, t23168, t23177, t40902, t4526, t51498, t51646, t51660, t51676, t51686, t5978, t820, t87714, t87729, t87764, t87775, t879);
        let t87920 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1632::<F>(t11008, t1559, t1579, t1580, t18699, t18714, t18800, t213, t225, t23244, t23245, t23383, t23404, t257, t2723, t2770, t2811, t39633, t39697, t39723, t40294, t41095, t4474, t4494, t4504, t4514, t4526, t51213, t51237, t51246, t51445, t51452, t51733, t6017, t6022, t6048, t6071, t6072, t62649, t62651, t62653, t62665, t62670, t62777, t62929, t63050, t63058, t63099, t76081, t76100, t76104, t76108, t76182, t76206, t76223, t77159, t77171, t77177, t77183, t77316, t820, t865, t868, t87395, t87400, t87629, t87786, t87798, t87824, t87850, t87869, t87895, t879);
        let t87931 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1633::<F>(t6075, t1544, t198, t207, t2403, t2411, t39537, t39540, t39741, t39744, t39747, t39750, t39756, t39760, t39764, t77460, t87318, t87342, t87357, t87373, t87640, t87920, t892);
        let t87942 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1634::<F>(t1583, t18268, t198, t23114, t2393, t39770, t39773, t39783, t39786, t39791, t39795, t4541, t5966, t87548, t87641, t87642, t87643, t87644, t87649, t892);
        let t87951 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1635::<F>(t1544, t18268, t2403, t39799, t39807, t39813, t39818, t39823, t40084, t40088, t40099, t40103, t4541, t5962, t77341, t87650, t87651);
        let t87952 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1636::<F>(t39989, t40115, t40131, t40137, t87655, t87658, t87660, t87661, t87662, t87663, t87666, t87667, t87668, t87669);
        let t87966 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1637::<F>(t18865, t1940, t198, t23279, t2403, t29598, t40067, t40072, t40167, t40171, t40184, t4541, t4546, t5962, t6079, t61033, t77333, t87670, t87671, t87673, t87674, t87675);
    (t87931, t87942, t87951, t87952, t87966)
}
