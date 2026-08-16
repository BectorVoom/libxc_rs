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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta644<F: Float>(t25796: F, t4547: F, t25425: F, t82431: F, t25816: F, t3173: F, t883: F, t25443: F, t1049: F, t7577: F, t7557: F, t82573: F, t1409: F, t14165: F, t23327: F, t23329: F, t23402: F, t25430: F, t25442: F, t25750: F, t25815: F, t3175: F, t6691: F, t82382: F, t82402: F, t82417: F, t82502: F, t23384: F, t25785: F, t25447: F, t1625: F, t6733: F, t23328: F, t6705: F, t13742: F, t1956: F, t23331: F, t23346: F, t23372: F, t23728: F, t25424: F, t25429: F, t25431: F, t25757: F, t25758: F, t25810: F, t4337: F, t4342: F, t4665: F, t50622: F, t6687: F, t82380: F, t23592: F, t225: F, t25791: F, t25413: F, t1598: F, t3014: F, t1921: F, t25403: F, t1066: F, t14658: F, t1599: F, t23332: F, t23365: F, t23594: F, t23722: F, t25784: F, t25797: F, t25826: F, t3010: F, t4660: F, t6704: F, t7553: F, t82400: F, t82426: F, t83424: F, t83453: F, t25749: F, t6698: F, t7566: F, t1052: F, t1065: F, t11010: F, t12648: F, t14529: F, t14545: F, t23313: F, t23369: F, t25406: F, t25731: F, t25778: F, t25811: F, t3174: F, t3207: F, t6776: F, t7600: F, t82432: F, t82436: F, t986: F, t14025: F, t23537: F, t13970: F, t23541: F, t13991: F, t14107: F, t14143: F, t14147: F, t14180: F, t14184: F, t14235: F, t23419: F, t23529: F, t4585: F, t4590: F, t6765: F, t82843: F, t82851: F, t83058: F, t83065: F, t13977: F, t13982: F, t13987: F, t14189: F, t23437: F, t4596: F, t4600: F, t4652: F, t82859: F, t82861: F, t82863: F, t82871: F, t82875: F, t82877: F, t83043: F, t83054: F, t83061: F, t4616: F, t6764: F, t23544: F, t4571: F, t23482: F, t25682: F, t25588: F, t344: F, t6740: F, t1046: F, t14093: F, t14174: F, t14230: F, t23483: F, t25679: F, t6747: F, t7583: F, t82883: F, t82885: F, t82893: F, t82897: F, t83114: F, t25580: F, t3053: F, t13961: F, t6755: F, t14202: F, t13950: F, t14215: F, t14491: F, t1622: F, t23454: F, t3064: F, t7578: F, t82914: F, t82941: F, t82944: F, t83016: F, t83038: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t88058, t88069, t88075, t88076, t88083, t88089, t88096) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2198::<F>(t25796, t4547, t25425, t82431, t25816, t3173, t883, t25443, t1049, t7577, t7557, t82573);
        let t88097 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2199::<F>(t1409, t14165, t23327, t23329, t23402, t25430, t25442, t25443, t25750, t25815, t3175, t6691, t7557, t82382, t82402, t82417, t82502, t88058, t88069, t88075, t88076, t88083, t88089, t88096);
        let t88137 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2200::<F>(t23384, t25785, t25447, t1625, t6733, t23328, t6705, t13742, t1956, t23327, t23331, t23346, t23372, t23728, t25424, t25429, t25431, t25757, t25758, t25810, t4337, t4342, t4665, t50622, t6687, t6691, t82380, t82502);
        let (t88155, t88179) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2201::<F>(t1625, t23592, t225, t25791, t23384, t25413, t1598, t3014, t1921, t7577, t25403, t1066, t14658, t1599, t23327, t23332, t23365, t23594, t23722, t25424, t25784, t25797, t25826, t3010, t4660, t6687, t6704, t6705, t7553, t82400, t82417, t82426, t83424, t83453);
        let t88213 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2202::<F>(t25749, t6698, t7566, t82573, t1052, t1065, t11010, t12648, t14529, t14545, t23313, t23329, t23346, t23369, t25406, t25429, t25430, t25731, t25778, t25811, t3174, t3207, t4665, t6687, t6776, t7600, t82382, t82432, t82436, t986);
        let t88254 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2203::<F>(t14025, t23537, t13970, t23541, t13991, t14107, t14143, t14147, t14180, t14184, t14235, t23419, t23529, t4585, t4590, t6765, t82843, t82851, t83058, t83065);
        let t88275 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2204::<F>(t13977, t13982, t13987, t14189, t23437, t23537, t4596, t4600, t4652, t6765, t82859, t82861, t82863, t82871, t82875, t82877, t83043, t83054, t83061);
        let t88303 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2205::<F>(t4616, t6764, t23544, t4571, t23482, t25682, t25588, t344, t6740, t1046, t14093, t14174, t14230, t23419, t23483, t25679, t6747, t6765, t7583, t82883, t82885, t82893, t82897, t83114);
        let t88327 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2206::<F>(t25580, t3053, t23529, t4571, t13961, t6755, t14202, t6765, t13950, t14215, t14491, t1622, t23454, t3064, t7578, t82914, t82941, t82944, t83016, t83038);
    (t88097, t88137, t88155, t88179, t88213, t88254, t88275, t88303, t88327)
}
