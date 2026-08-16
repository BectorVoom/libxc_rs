//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta692 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2635;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2636;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2637;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2638;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2639;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2640;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2641;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta692<F: Float>(t15831: F, t225: F, t11606: F, t11607: F, t11613: F, t11621: F, t11625: F, t11720: F, t11869: F, t11871: F, t11872: F, t11877: F, t11881: F, t11883: F, t11884: F, t11888: F, t11889: F, t11893: F, t11904: F, t11907: F, t11916: F, t11925: F, t11935: F, t1201: F, t1215: F, t1238: F, t1241: F, t1244: F, t1246: F, t1249: F, t1252: F, t14986: F, t14992: F, t15000: F, t15001: F, t15004: F, t15009: F, t15016: F, t15019: F, t15022: F, t15027: F, t15032: F, t15240: F, t15245: F, t15247: F, t15253: F, t15257: F, t15426: F, t15771: F, t15772: F, t15790: F, t15794: F, t15820: F, t1729: F, t1755: F, t1756: F, t1760: F, t3565: F, t3593: F, t3598: F, t3599: F, t3604: F, t3610: F, t3612: F, t3613: F, t3617: F, t3624: F, t3628: F, t3630: F, t3631: F, t44698: F, t44701: F, t44707: F, t44724: F, t44726: F, t44753: F, t44754: F, t44906: F, t45320: F, t45326: F, t45350: F, t491: F, t4964: F, t498: F, t5055: F, t5060: F, t5064: F, t5068: F, t5069: F, t5072: F, t5075: F, t5076: F, t5086: F, t5088: F, t5089: F, t52377: F, t52386: F, t52424: F, t52471: F, t52479: F, t52480: F, t52485: F, t52500: F, t52554: F, t53538: F, t53545: F, t53590: F, t53592: F, t53613: F, t53650: F, t11605: F, t11868: F, t1190: F, t11918: F, t11919: F, t11928: F, t11934: F, t14972: F, t15787: F, t1720: F, t1761: F, t27784: F, t3487: F, t3590: F, t45345: F, t45355: F, t45375: F, t4940: F, t15816: F, t11608: F, t1235: F, t14980: F, t15425: F, t15797: F, t15803: F, t3481: F, t3600: F, t466: F, t4945: F, t5052: F, t53529: F, t11944: F, t1256: F, t14696: F, t15838: F, t1763: F, t193: F, t336: F, t3633: F, t43706: F, t4700: F, t51889: F, t51892: F, t51898: F, t51903: F, t51905: F, t51906: F, t51913: F, t51916: F, t51946: F, t28: F, t265: F, t504: F, t47655: F, t51129: F, t51803: F, t51825: F, t51826: F, t51836: F, t51867: F, t51885: F, t10150: F, t1081: F, t11122: F, t11957: F, t1260: F, t12606: F, t13493: F, t1409: F, t1534: F, t15844: F, t1649: F, t1768: F, t2250: F, t3231: F, t3644: F, t3966: F, t4324: F, t45872: F, t47668: F, t47670: F, t47672: F, t47674: F, t47676: F, t506: F, t5099: F, t52: F, t607: F, t9258: F, dens_threshold: F, rho1: F, zeta_threshold: F, t113: F, t12504: F, t12507: F, t12545: F, t12557: F, t1271: F, t12841: F, t16503: F, t2314: F, t2320: F, t2363: F, t4028: F, t4034: F, t4073: F, t4077: F, t45782: F, t46118: F, t50803: F, t510: F, t5107: F, t574: F, t652: F, t9348: F, t15908: F, t9467: F, t9882: F, t118: F, t2375: F, t5151: F, t16169: F, t2663: F, t1388: F, t3734: F, t15892: F, t2371: F) -> (F, F, F, F, F, F, F) {
        let t53665 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2635::<F>(t15831, t225, t11606, t11607, t11613, t11621, t11625, t11720, t11869, t11871, t11872, t11877, t11881, t11883, t11884, t11888, t11889, t11893, t11904, t11907, t11916, t11925, t11935, t1201, t1215, t1238, t1241, t1244, t1246, t1249, t1252, t14986, t14992, t15000, t15001, t15004, t15009, t15016, t15019, t15022, t15027, t15032, t15240, t15245, t15247, t15253, t15257, t15426, t15771, t15772, t15790, t15794, t15820, t1729, t1755, t1756, t1760, t3565, t3593, t3598, t3599, t3604, t3610, t3612, t3613, t3617, t3624, t3628, t3630, t3631, t44698, t44701, t44707, t44724, t44726, t44753, t44754, t44906, t45320, t45326, t45350, t491, t4964, t498, t5055, t5060, t5064, t5068, t5069, t5072, t5075, t5076, t5086, t5088, t5089, t52377, t52386, t52424, t52471, t52479, t52480, t52485, t52500, t52554, t53538, t53545, t53590, t53592, t53613, t53650);
        let t53697 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2636::<F>(t11605, t1760, t11868, t1190, t11918, t11919, t11928, t11934, t1238, t14972, t15771, t15787, t15790, t1720, t1761, t27784, t3487, t3590, t3593, t3598, t3631, t45345, t45355, t45375, t4940, t498, t5055, t5089);
        let t53729 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2637::<F>(t15816, t225, t11608, t11613, t11925, t11928, t1235, t1252, t14980, t15425, t15787, t15797, t15803, t3481, t3487, t3593, t3600, t3631, t466, t4945, t498, t5052, t5055, t5060, t5089, t53529);
        let t53735 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2638::<F>(t11944, t1256, t14696, t15838, t1763, t193, t336, t3633, t43706, t4700, t51889, t51892, t51898, t51903, t51905, t51906, t51913, t51916, t51946, t53665, t53697, t53729);
        let t53757 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2639::<F>(t28, t265, t504, t47655, t51129, t51803, t51825, t51826, t51836, t51867, t51885, t53735, t10150, t1081, t11122, t11957, t1260, t12606, t13493, t1409, t1534, t15844, t1649, t1768, t2250, t3231, t3644, t3966, t4324, t45872, t47668, t47670, t47672, t47674, t47676, t506, t5099, t52, t607, t9258, dens_threshold, rho1, zeta_threshold);
        let t53774 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2640::<F>(t113, t12504, t12507, t12545, t12557, t1271, t12841, t16503, t2314, t2320, t2363, t4028, t4034, t4073, t4077, t45782, t46118, t50803, t510, t5107, t53757, t574, t652, t9348);
        let (t53778, t53780, t53783, t53788, t53789, t53796) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2641::<F>(t15908, t9467, t9882, t118, t2375, t5151, t16169, t2663, t1388, t3734, t15892, t2371);
    (t53774, t53778, t53780, t53783, t53788, t53789, t53796)
}
