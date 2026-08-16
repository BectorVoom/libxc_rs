//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta644 (260520-c91 hierarchical CSE).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2198;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2199;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2200;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2201;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2202;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2203;
use chunk6::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2204;
use chunk7::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2205;
use chunk8::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2206;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta644(t25796: f64, t4547: f64, t25425: f64, t82431: f64, t25816: f64, t3173: f64, t883: f64, t25443: f64, t1049: f64, t7577: f64, t7557: f64, t82573: f64, t1409: f64, t14165: f64, t23327: f64, t23329: f64, t23402: f64, t25430: f64, t25442: f64, t25750: f64, t25815: f64, t3175: f64, t6691: f64, t82382: f64, t82402: f64, t82417: f64, t82502: f64, t23384: f64, t25785: f64, t25447: f64, t1625: f64, t6733: f64, t23328: f64, t6705: f64, t13742: f64, t1956: f64, t23331: f64, t23346: f64, t23372: f64, t23728: f64, t25424: f64, t25429: f64, t25431: f64, t25757: f64, t25758: f64, t25810: f64, t4337: f64, t4342: f64, t4665: f64, t50622: f64, t6687: f64, t82380: f64, t23592: f64, t225: f64, t25791: f64, t25413: f64, t1598: f64, t3014: f64, t1921: f64, t25403: f64, t1066: f64, t14658: f64, t1599: f64, t23332: f64, t23365: f64, t23594: f64, t23722: f64, t25784: f64, t25797: f64, t25826: f64, t3010: f64, t4660: f64, t6704: f64, t7553: f64, t82400: f64, t82426: f64, t83424: f64, t83453: f64, t25749: f64, t6698: f64, t7566: f64, t1052: f64, t1065: f64, t11010: f64, t12648: f64, t14529: f64, t14545: f64, t23313: f64, t23369: f64, t25406: f64, t25731: f64, t25778: f64, t25811: f64, t3174: f64, t3207: f64, t6776: f64, t7600: f64, t82432: f64, t82436: f64, t986: f64, t14025: f64, t23537: f64, t13970: f64, t23541: f64, t13991: f64, t14107: f64, t14143: f64, t14147: f64, t14180: f64, t14184: f64, t14235: f64, t23419: f64, t23529: f64, t4585: f64, t4590: f64, t6765: f64, t82843: f64, t82851: f64, t83058: f64, t83065: f64, t13977: f64, t13982: f64, t13987: f64, t14189: f64, t23437: f64, t4596: f64, t4600: f64, t4652: f64, t82859: f64, t82861: f64, t82863: f64, t82871: f64, t82875: f64, t82877: f64, t83043: f64, t83054: f64, t83061: f64, t4616: f64, t6764: f64, t23544: f64, t4571: f64, t23482: f64, t25682: f64, t25588: f64, t344: f64, t6740: f64, t1046: f64, t14093: f64, t14174: f64, t14230: f64, t23483: f64, t25679: f64, t6747: f64, t7583: f64, t82883: f64, t82885: f64, t82893: f64, t82897: f64, t83114: f64, t25580: f64, t3053: f64, t13961: f64, t6755: f64, t14202: f64, t13950: f64, t14215: f64, t14491: f64, t1622: f64, t23454: f64, t3064: f64, t7578: f64, t82914: f64, t82941: f64, t82944: f64, t83016: f64, t83038: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t88058, t88069, t88075, t88076, t88083, t88089, t88096) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2198(t25796, t4547, t25425, t82431, t25816, t3173, t883, t25443, t1049, t7577, t7557, t82573);
        let t88097 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2199(t1409, t14165, t23327, t23329, t23402, t25430, t25442, t25443, t25750, t25815, t3175, t6691, t7557, t82382, t82402, t82417, t82502, t88058, t88069, t88075, t88076, t88083, t88089, t88096);
        let t88137 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2200(t23384, t25785, t25447, t1625, t6733, t23328, t6705, t13742, t1956, t23327, t23331, t23346, t23372, t23728, t25424, t25429, t25431, t25757, t25758, t25810, t4337, t4342, t4665, t50622, t6687, t6691, t82380, t82502);
        let (t88155, t88179) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2201(t1625, t23592, t225, t25791, t23384, t25413, t1598, t3014, t1921, t7577, t25403, t1066, t14658, t1599, t23327, t23332, t23365, t23594, t23722, t25424, t25784, t25797, t25826, t3010, t4660, t6687, t6704, t6705, t7553, t82400, t82417, t82426, t83424, t83453);
        let t88213 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2202(t25749, t6698, t7566, t82573, t1052, t1065, t11010, t12648, t14529, t14545, t23313, t23329, t23346, t23369, t25406, t25429, t25430, t25731, t25778, t25811, t3174, t3207, t4665, t6687, t6776, t7600, t82382, t82432, t82436, t986);
        let t88254 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2203(t14025, t23537, t13970, t23541, t13991, t14107, t14143, t14147, t14180, t14184, t14235, t23419, t23529, t4585, t4590, t6765, t82843, t82851, t83058, t83065);
        let t88275 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2204(t13977, t13982, t13987, t14189, t23437, t23537, t4596, t4600, t4652, t6765, t82859, t82861, t82863, t82871, t82875, t82877, t83043, t83054, t83061);
        let t88303 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2205(t4616, t6764, t23544, t4571, t23482, t25682, t25588, t344, t6740, t1046, t14093, t14174, t14230, t23419, t23483, t25679, t6747, t6765, t7583, t82883, t82885, t82893, t82897, t83114);
        let t88327 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2206(t25580, t3053, t23529, t4571, t13961, t6755, t14202, t6765, t13950, t14215, t14491, t1622, t23454, t3064, t7578, t82914, t82941, t82944, t83016, t83038);
    (t88097, t88137, t88155, t88179, t88213, t88254, t88275, t88303, t88327)
}
