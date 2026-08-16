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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta550(t40325: f64, t87399: f64, t10871: f64, t10698: f64, t10870: f64, t1544: f64, t23148: f64, t2477: f64, t2721: f64, t40324: f64, t51170: f64, t5962: f64, t5966: f64, t62251: f64, t62399: f64, t62401: f64, t62431: f64, t62443: f64, t62445: f64, t76878: f64, t76882: f64, t76887: f64, t77127: f64, t77131: f64, t825: f64, t827: f64, t828: f64, t851: f64, t87395: f64, t87400: f64, t87470: f64, t87503: f64, t87562: f64, t87579: f64, t87608: f64, t87721: f64, t87742: f64, t213: f64, t234: f64, t62633: f64, t76117: f64, t76125: f64, t76134: f64, t76139: f64, t76144: f64, t76153: f64, t76158: f64, t76163: f64, t76172: f64, t14546: f64, t18677: f64, t39649: f64, t39652: f64, t51390: f64, t51403: f64, t51408: f64, t62684: f64, t62716: f64, t62723: f64, t76237: f64, t76242: f64, t76255: f64, t14961: f64, t1559: f64, t23172: f64, t40314: f64, t40316: f64, t4514: f64, t51553: f64, t62843: f64, t62847: f64, t62874: f64, t62907: f64, t76127: f64, t77191: f64, t77197: f64, t820: f64, t14586: f64, t4504: f64, t51578: f64, t51635: f64, t6017: f64, t62909: f64, t62920: f64, t62922: f64, t62952: f64, t62983: f64, t62999: f64, t77159: f64, t77225: f64, t10952: f64, t18714: f64, t23168: f64, t23177: f64, t40902: f64, t4526: f64, t51498: f64, t51646: f64, t51660: f64, t51676: f64, t51686: f64, t5978: f64, t87714: f64, t87729: f64, t879: f64, t11008: f64, t1579: f64, t1580: f64, t18699: f64, t18800: f64, t225: f64, t23244: f64, t23245: f64, t23383: f64, t23404: f64, t257: f64, t2723: f64, t2770: f64, t2811: f64, t39633: f64, t39697: f64, t39723: f64, t40294: f64, t41095: f64, t4474: f64, t4494: f64, t51213: f64, t51237: f64, t51246: f64, t51445: f64, t51452: f64, t51733: f64, t6022: f64, t6048: f64, t6071: f64, t6072: f64, t62649: f64, t62651: f64, t62653: f64, t62665: f64, t62670: f64, t62777: f64, t62929: f64, t63050: f64, t63058: f64, t63099: f64, t76081: f64, t76100: f64, t76104: f64, t76108: f64, t76182: f64, t76206: f64, t76223: f64, t77171: f64, t77177: f64, t77183: f64, t77316: f64, t865: f64, t868: f64, t87629: f64, t6075: f64, t198: f64, t207: f64, t2403: f64, t2411: f64, t39537: f64, t39540: f64, t39741: f64, t39744: f64, t39747: f64, t39750: f64, t39756: f64, t39760: f64, t39764: f64, t77460: f64, t87318: f64, t87342: f64, t87357: f64, t87373: f64, t87640: f64, t892: f64, t1583: f64, t18268: f64, t23114: f64, t2393: f64, t39770: f64, t39773: f64, t39783: f64, t39786: f64, t39791: f64, t39795: f64, t4541: f64, t87548: f64, t87641: f64, t87642: f64, t87643: f64, t87644: f64, t87649: f64, t39799: f64, t39807: f64, t39813: f64, t39818: f64, t39823: f64, t40084: f64, t40088: f64, t40099: f64, t40103: f64, t77341: f64, t87650: f64, t87651: f64, t39989: f64, t40115: f64, t40131: f64, t40137: f64, t87655: f64, t87658: f64, t87660: f64, t87661: f64, t87662: f64, t87663: f64, t87666: f64, t87667: f64, t87668: f64, t87669: f64, t18865: f64, t1940: f64, t23279: f64, t29598: f64, t40067: f64, t40072: f64, t40167: f64, t40171: f64, t40184: f64, t4546: f64, t6079: f64, t61033: f64, t77333: f64, t87670: f64, t87671: f64, t87673: f64, t87674: f64, t87675: f64) -> (f64, f64, f64, f64, f64) {
        let (t87764, t87775, t87783) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1626(t40325, t87399, t10871, t10698, t10870, t1544, t23148, t2477, t2721, t40324, t51170, t5962, t5966, t62251, t62399, t62401, t62431, t62443, t62445, t76878, t76882, t76887, t77127, t77131, t825, t827, t828, t851, t87395, t87400);
        let (t87786, t87798) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1627(t87470, t87503, t87562, t87579, t87608, t87721, t87742, t87783, t213, t234, t62633, t76117, t76125, t76134, t76139, t76144, t76153, t76158, t76163, t76172);
        let t87824 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1628(t14546, t18677, t39649, t39652, t51390, t51403, t51408, t62684, t62716, t62723, t76237, t76242, t76255);
        let t87850 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1629(t14961, t1559, t23172, t40314, t40316, t4514, t51553, t62843, t62847, t62874, t62907, t76127, t77191, t77197, t820);
        let t87869 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1630(t14586, t1559, t18677, t4504, t4514, t51578, t51635, t6017, t62909, t62920, t62922, t62952, t62983, t62999, t77159, t77225, t820);
        let t87895 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1631(t10952, t18714, t23168, t23177, t40902, t4526, t51498, t51646, t51660, t51676, t51686, t5978, t820, t87714, t87729, t87764, t87775, t879);
        let t87920 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1632(t11008, t1559, t1579, t1580, t18699, t18714, t18800, t213, t225, t23244, t23245, t23383, t23404, t257, t2723, t2770, t2811, t39633, t39697, t39723, t40294, t41095, t4474, t4494, t4504, t4514, t4526, t51213, t51237, t51246, t51445, t51452, t51733, t6017, t6022, t6048, t6071, t6072, t62649, t62651, t62653, t62665, t62670, t62777, t62929, t63050, t63058, t63099, t76081, t76100, t76104, t76108, t76182, t76206, t76223, t77159, t77171, t77177, t77183, t77316, t820, t865, t868, t87395, t87400, t87629, t87786, t87798, t87824, t87850, t87869, t87895, t879);
        let t87931 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1633(t6075, t1544, t198, t207, t2403, t2411, t39537, t39540, t39741, t39744, t39747, t39750, t39756, t39760, t39764, t77460, t87318, t87342, t87357, t87373, t87640, t87920, t892);
        let t87942 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1634(t1583, t18268, t198, t23114, t2393, t39770, t39773, t39783, t39786, t39791, t39795, t4541, t5966, t87548, t87641, t87642, t87643, t87644, t87649, t892);
        let t87951 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1635(t1544, t18268, t2403, t39799, t39807, t39813, t39818, t39823, t40084, t40088, t40099, t40103, t4541, t5962, t77341, t87650, t87651);
        let t87952 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1636(t39989, t40115, t40131, t40137, t87655, t87658, t87660, t87661, t87662, t87663, t87666, t87667, t87668, t87669);
        let t87966 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1637(t18865, t1940, t198, t23279, t2403, t29598, t40067, t40072, t40167, t40171, t40184, t4541, t4546, t5962, t6079, t61033, t77333, t87670, t87671, t87673, t87674, t87675);
    (t87931, t87942, t87951, t87952, t87966)
}
