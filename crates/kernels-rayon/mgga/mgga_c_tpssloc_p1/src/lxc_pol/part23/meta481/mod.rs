//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta481 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1439;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1440;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1441;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1442;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1443;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1444;
use chunk6::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1445;
use chunk7::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1446;
use chunk8::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1447;
use chunk9::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1448;
use chunk10::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1449;
use chunk11::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1450;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta481(t15376: f64, t22069: f64, t3447: f64, t4908: f64, t6123: f64, t64811: f64, t73274: f64, t73276: f64, t73279: f64, t73287: f64, t73290: f64, t73307: f64, t73314: f64, t78043: f64, t78047: f64, t4900: f64, t4904: f64, t64821: f64, t73169: f64, t73330: f64, t73386: f64, t73389: f64, t73395: f64, t73417: f64, t73420: f64, t73424: f64, t78031: f64, t78039: f64, t15390: f64, t18409: f64, t18420: f64, t18427: f64, t18469: f64, t22072: f64, t22075: f64, t22090: f64, t22095: f64, t4919: f64, t52081: f64, t64648: f64, t73181: f64, t73201: f64, t73405: f64, t73427: f64, t1740: f64, t48: f64, t338: f64, t11546: f64, t1174: f64, t18321: f64, t44566: f64, t463: f64, t52124: f64, t6127: f64, t64878: f64, t64881: f64, t64885: f64, t64979: f64, t73433: f64, t73444: f64, t73451: f64, t75836: f64, sigma2: f64, t1177: f64, t1714: f64, t22032: f64, t22047: f64, t22052: f64, t22082: f64, t3440: f64, t3441: f64, t3455: f64, t44487: f64, t44621: f64, t44622: f64, t460: f64, t4889: f64, t4934: f64, t6120: f64, t65002: f64, t65023: f64, t73491: f64, t75847: f64, t6144: f64, t6138: f64, t1409: f64, t1710: f64, t22035: f64, t22041: f64, t22056: f64, t22060: f64, t3450: f64, t457: f64, t6131: f64, t65112: f64, t65126: f64, t73113: f64, t974: f64, t50846: f64, t63888: f64, t63893: f64, t63911: f64, t71335: f64, t71337: f64, t71408: f64, t77959: f64, t77963: f64, t77967: f64, t78084: f64, t44466: f64, t71470: f64, t71472: f64, t71474: f64, t77971: f64, t77975: f64, t77979: f64, t77983: f64, t78087: f64, t78090: f64, t78093: f64, t78100: f64, t11516: f64, t11547: f64, t1178: f64, t1717: f64, t29614: f64, t52281: f64, t6141: f64, t6147: f64, t73523: f64, t73535: f64, t73541: f64, t75912: f64, t78423: f64, t1238: f64, t1751: f64, t1760: f64, t1761: f64, t19232: f64, t19234: f64, t22004: f64, t22008: f64, t22113: f64, t22393: f64, t22394: f64, t3598: f64, t491: f64, t4945: f64, t498: f64, t5055: f64, t6150: f64, t6238: f64, t6244: f64, t6268: f64, t73900: f64, t78379: f64, t11678: f64, t1227: f64, t15507: f64, t15654: f64, t1653: f64, t1734: f64, t1737: f64, t1748: f64, t19033: f64, t22275: f64, t22301: f64, t3578: f64, t4582: f64, t4972: f64, t53087: f64, t6211: f64, t65444: f64, t65464: f64, t72161: f64, t72181: f64, t72183: f64, t72389: f64, t72398: f64, t72967: f64, t77606: f64, t77621: f64, t11692: f64, t18395: f64, t19047: f64, t22208: f64, t22246: f64, t22258: f64, t22314: f64, t5005: f64, t5019: f64, t5024: f64, t53083: f64, t6221: f64, t65528: f64, t72223: f64, t72225: f64, t72229: f64, t72248: f64, t72251: f64, t72253: f64, t72384: f64, t72767: f64, t15453: f64, t1730: f64, t22174: f64, t488: f64, t6232: f64, t65552: f64, t65558: f64, t65581: f64, t65706: f64, t72273: f64, t72285: f64, t72287: f64, t72289: f64, t72293: f64, t72297: f64, t72302: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let t78441 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1439(t15376, t22069, t3447, t4908, t6123, t64811, t73274, t73276, t73279, t73287, t73290, t73307, t73314, t78043, t78047);
        let t78460 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1440(t3447, t4900, t4904, t64821, t73169, t73330, t73386, t73389, t73395, t73417, t73420, t73424, t78031, t78039);
        let t78489 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1441(t15376, t15390, t18409, t18420, t18427, t18469, t22072, t22075, t22090, t22095, t3447, t4904, t4919, t52081, t64648, t73181, t73201, t73405, t73427);
        let (t78505, t78506, t78516) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1442(t1740, t48, t338, t11546, t1174, t15390, t18321, t3447, t44566, t463, t4919, t52124, t6127, t64878, t64881, t64885, t64979, t73433, t73444, t73451, t75836, sigma2);
        let t78545 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1443(t1174, t1177, t1714, t18321, t22032, t22047, t22052, t22082, t3440, t3441, t3455, t44487, t44621, t44622, t460, t4889, t4934, t6120, t65002, t65023, t73491, t75836, t75847);
        let t78578 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1444(t6144, t6138, t1174, t1409, t1710, t18321, t22035, t22041, t22056, t22060, t3447, t3450, t457, t460, t4889, t4919, t6131, t65112, t65126, t73113, t974);
        let (t78596, t78607) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1445(t50846, t63888, t63893, t63911, t71335, t71337, t71408, t77959, t77963, t77967, t78084, t44466, t71470, t71472, t71474, t77971, t77975, t77979, t77983, t78087, t78090, t78093, t78100);
        let t78634 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1446(t11516, t11547, t1174, t1177, t1178, t1717, t18321, t29614, t3440, t457, t460, t4934, t52281, t6138, t6141, t6147, t73113, t73523, t73535, t73541, t75836, t75912, t78596, t78607, t974);
        let (t78637, t78646) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1447(t78423, t78441, t78460, t78489, t78516, t78545, t78578, t78634, t1238, t1751, t1760, t1761, t19232, t19234, t22004, t22008, t22113, t22393, t22394, t3598, t491, t4945, t498, t5055, t6150, t6238, t6244, t6268, t73900, t78379);
        let t78689 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1448(t11678, t1227, t15507, t15654, t1653, t1734, t1737, t1748, t19033, t22275, t22301, t3578, t4582, t4972, t53087, t6211, t65444, t65464, t72161, t72181, t72183, t72389, t72398, t72967, t77606, t77621);
        let t78713 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1449(t11692, t1748, t18395, t19047, t22208, t22246, t22258, t22314, t3578, t5005, t5019, t5024, t53083, t6221, t65528, t72223, t72225, t72229, t72248, t72251, t72253, t72384, t72767);
        let t78734 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1450(t1227, t15453, t1730, t22174, t4582, t488, t6232, t65552, t65558, t65581, t65706, t72273, t72285, t72287, t72289, t72293, t72297, t72302, t77606);
    (t78505, t78506, t78637, t78646, t78689, t78713, t78734)
}
