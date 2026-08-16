//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta692 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2635;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2636;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2637;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2638;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2639;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2640;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2641;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta692(t15831: f64, t225: f64, t11606: f64, t11607: f64, t11613: f64, t11621: f64, t11625: f64, t11720: f64, t11869: f64, t11871: f64, t11872: f64, t11877: f64, t11881: f64, t11883: f64, t11884: f64, t11888: f64, t11889: f64, t11893: f64, t11904: f64, t11907: f64, t11916: f64, t11925: f64, t11935: f64, t1201: f64, t1215: f64, t1238: f64, t1241: f64, t1244: f64, t1246: f64, t1249: f64, t1252: f64, t14986: f64, t14992: f64, t15000: f64, t15001: f64, t15004: f64, t15009: f64, t15016: f64, t15019: f64, t15022: f64, t15027: f64, t15032: f64, t15240: f64, t15245: f64, t15247: f64, t15253: f64, t15257: f64, t15426: f64, t15771: f64, t15772: f64, t15790: f64, t15794: f64, t15820: f64, t1729: f64, t1755: f64, t1756: f64, t1760: f64, t3565: f64, t3593: f64, t3598: f64, t3599: f64, t3604: f64, t3610: f64, t3612: f64, t3613: f64, t3617: f64, t3624: f64, t3628: f64, t3630: f64, t3631: f64, t44698: f64, t44701: f64, t44707: f64, t44724: f64, t44726: f64, t44753: f64, t44754: f64, t44906: f64, t45320: f64, t45326: f64, t45350: f64, t491: f64, t4964: f64, t498: f64, t5055: f64, t5060: f64, t5064: f64, t5068: f64, t5069: f64, t5072: f64, t5075: f64, t5076: f64, t5086: f64, t5088: f64, t5089: f64, t52377: f64, t52386: f64, t52424: f64, t52471: f64, t52479: f64, t52480: f64, t52485: f64, t52500: f64, t52554: f64, t53538: f64, t53545: f64, t53590: f64, t53592: f64, t53613: f64, t53650: f64, t11605: f64, t11868: f64, t1190: f64, t11918: f64, t11919: f64, t11928: f64, t11934: f64, t14972: f64, t15787: f64, t1720: f64, t1761: f64, t27784: f64, t3487: f64, t3590: f64, t45345: f64, t45355: f64, t45375: f64, t4940: f64, t15816: f64, t11608: f64, t1235: f64, t14980: f64, t15425: f64, t15797: f64, t15803: f64, t3481: f64, t3600: f64, t466: f64, t4945: f64, t5052: f64, t53529: f64, t11944: f64, t1256: f64, t14696: f64, t15838: f64, t1763: f64, t193: f64, t336: f64, t3633: f64, t43706: f64, t4700: f64, t51889: f64, t51892: f64, t51898: f64, t51903: f64, t51905: f64, t51906: f64, t51913: f64, t51916: f64, t51946: f64, t28: f64, t265: f64, t504: f64, t47655: f64, t51129: f64, t51803: f64, t51825: f64, t51826: f64, t51836: f64, t51867: f64, t51885: f64, t10150: f64, t1081: f64, t11122: f64, t11957: f64, t1260: f64, t12606: f64, t13493: f64, t1409: f64, t1534: f64, t15844: f64, t1649: f64, t1768: f64, t2250: f64, t3231: f64, t3644: f64, t3966: f64, t4324: f64, t45872: f64, t47668: f64, t47670: f64, t47672: f64, t47674: f64, t47676: f64, t506: f64, t5099: f64, t52: f64, t607: f64, t9258: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64, t113: f64, t12504: f64, t12507: f64, t12545: f64, t12557: f64, t1271: f64, t12841: f64, t16503: f64, t2314: f64, t2320: f64, t2363: f64, t4028: f64, t4034: f64, t4073: f64, t4077: f64, t45782: f64, t46118: f64, t50803: f64, t510: f64, t5107: f64, t574: f64, t652: f64, t9348: f64, t15908: f64, t9467: f64, t9882: f64, t118: f64, t2375: f64, t5151: f64, t16169: f64, t2663: f64, t1388: f64, t3734: f64, t15892: f64, t2371: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let t53665 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2635(t15831, t225, t11606, t11607, t11613, t11621, t11625, t11720, t11869, t11871, t11872, t11877, t11881, t11883, t11884, t11888, t11889, t11893, t11904, t11907, t11916, t11925, t11935, t1201, t1215, t1238, t1241, t1244, t1246, t1249, t1252, t14986, t14992, t15000, t15001, t15004, t15009, t15016, t15019, t15022, t15027, t15032, t15240, t15245, t15247, t15253, t15257, t15426, t15771, t15772, t15790, t15794, t15820, t1729, t1755, t1756, t1760, t3565, t3593, t3598, t3599, t3604, t3610, t3612, t3613, t3617, t3624, t3628, t3630, t3631, t44698, t44701, t44707, t44724, t44726, t44753, t44754, t44906, t45320, t45326, t45350, t491, t4964, t498, t5055, t5060, t5064, t5068, t5069, t5072, t5075, t5076, t5086, t5088, t5089, t52377, t52386, t52424, t52471, t52479, t52480, t52485, t52500, t52554, t53538, t53545, t53590, t53592, t53613, t53650);
        let t53697 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2636(t11605, t1760, t11868, t1190, t11918, t11919, t11928, t11934, t1238, t14972, t15771, t15787, t15790, t1720, t1761, t27784, t3487, t3590, t3593, t3598, t3631, t45345, t45355, t45375, t4940, t498, t5055, t5089);
        let t53729 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2637(t15816, t225, t11608, t11613, t11925, t11928, t1235, t1252, t14980, t15425, t15787, t15797, t15803, t3481, t3487, t3593, t3600, t3631, t466, t4945, t498, t5052, t5055, t5060, t5089, t53529);
        let t53735 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2638(t11944, t1256, t14696, t15838, t1763, t193, t336, t3633, t43706, t4700, t51889, t51892, t51898, t51903, t51905, t51906, t51913, t51916, t51946, t53665, t53697, t53729);
        let t53757 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2639(t28, t265, t504, t47655, t51129, t51803, t51825, t51826, t51836, t51867, t51885, t53735, t10150, t1081, t11122, t11957, t1260, t12606, t13493, t1409, t1534, t15844, t1649, t1768, t2250, t3231, t3644, t3966, t4324, t45872, t47668, t47670, t47672, t47674, t47676, t506, t5099, t52, t607, t9258, dens_threshold, rho1, zeta_threshold);
        let t53774 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2640(t113, t12504, t12507, t12545, t12557, t1271, t12841, t16503, t2314, t2320, t2363, t4028, t4034, t4073, t4077, t45782, t46118, t50803, t510, t5107, t53757, t574, t652, t9348);
        let (t53778, t53780, t53783, t53788, t53789, t53796) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2641(t15908, t9467, t9882, t118, t2375, t5151, t16169, t2663, t1388, t3734, t15892, t2371);
    (t53774, t53778, t53780, t53783, t53788, t53789, t53796)
}
