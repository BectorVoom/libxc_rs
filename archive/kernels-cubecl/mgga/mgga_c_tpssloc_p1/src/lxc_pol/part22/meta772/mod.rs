//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta772 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2632;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2633;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2634;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2635;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2636;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2637;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2638;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2639;
use chunk8::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2640;
use chunk9::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2641;
use chunk10::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2642;
use chunk11::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2643;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta772<F: Float>(t3447: F, t4904: F, t64779: F, t15402: F, t21749: F, t15376: F, t15382: F, t15390: F, t15395: F, t18543: F, t18546: F, t44635: F, t458: F, t4900: F, t4919: F, t4936: F, t52100: F, t52368: F, t6138: F, t64644: F, t64870: F, t65018: F, t65056: F, t71193: F, t72688: F, t1090: F, t11569: F, t1174: F, t1177: F, t1178: F, t1184: F, t15293: F, t18321: F, t18404: F, t18416: F, t18417: F, t18420: F, t18421: F, t18431: F, t18559: F, t18564: F, t22038: F, t29614: F, t3449: F, t3451: F, t44487: F, t44504: F, t457: F, t460: F, t4889: F, t4901: F, t4908: F, t4913: F, t4928: F, t4934: F, t51971: F, t51981: F, t52096: F, t52110: F, t52124: F, t52281: F, t52288: F, t52355: F, t5971: F, t5975: F, t64667: F, t64686: F, t64696: F, t64699: F, t64702: F, t64811: F, t64858: F, t64878: F, t64881: F, t64885: F, t64994: F, t65002: F, t65008: F, t65023: F, t65087: F, t65128: F, t65136: F, t65139: F, t65142: F, t67060: F, t71133: F, t71138: F, t71158: F, t71164: F, t72788: F, t7319: F, t73192: F, t73199: F, t73201: F, t73252: F, t73272: F, t73274: F, t73276: F, t73279: F, t73316: F, t73330: F, t73355: F, t73369: F, t73399: F, t73439: F, t73444: F, t73451: F, t73480: F, t73491: F, t73496: F, t73525: F, t8034: F, t974: F, t225: F, t11665: F, t11668: F, t11678: F, t1215: F, t15659: F, t19083: F, t22162: F, t3577: F, t3578: F, t45296: F, t484: F, t488: F, t4965: F, t4974: F, t5012: F, t52893: F, t53516: F, t53520: F, t5979: F, t6164: F, t66646: F, t66648: F, t66668: F, t68: F, t73138: F, t73142: F, t72180: F, t72233: F, t72268: F, t72299: F, t72333: F, t72357: F, t72380: F, t72405: F, t72452: F, t72484: F, t72522: F, t72552: F, t72593: F, t72622: F, t72654: F, t72683: F, t72712: F, t72735: F, t72783: F, t72823: F, t72842: F, t72878: F, t72911: F, t72938: F, t72970: F, t72996: F, t73019: F, t73048: F, t73078: F, t73108: F, t73126: F, t22398: F, t1243: F, t72361: F, t1235: F, t22298: F, t11907: F, t11914: F, t11915: F, t15027: F, t15245: F, t19128: F, t19129: F, t19131: F, t19142: F, t19157: F, t19160: F, t22341: F, t22348: F, t22354: F, t22372: F, t22390: F, t3604: F, t3624: F, t44724: F, t44726: F, t5064: F, t53565: F, t5052: F, t6224: F, t11881: F, t1244: F, t1246: F, t19165: F, t19201: F, t22340: F, t22358: F, t22364: F, t22368: F, t22386: F, t3610: F, t3625: F, t44698: F, t44701: F, t44753: F, t44754: F, t45326: F, t491: F, t5068: F, t5072: F, t5084: F, t6218: F, t72217: F, t6739: F, t15032: F, t1756: F, t19154: F, t19170: F, t19189: F, t19190: F, t19203: F, t22389: F, t470: F, t493: F, t494: F, t5069: F, t5079: F, t53592: F, t6256: F, t6261: F, t65254: F, t66787: F, t11883: F, t11904: F, t1755: F, t18940: F, t19146: F, t22243: F, t22365: F, t23508: F, t3612: F, t44785: F, t475: F, t4964: F, t5073: F, t5076: F, t52435: F, t6263: F, t6265: F, t11888: F, t11889: F, t1190: F, t1201: F, t1238: F, t1241: F, t1247: F, t1249: F, t1252: F, t14972: F, t14980: F, t15700: F, t15797: F, t1729: F, t1734: F, t1751: F, t1758: F, t18572: F, t19120: F, t19123: F, t19138: F, t19139: F, t19153: F, t19156: F, t19166: F, t19174: F, t19176: F, t19180: F, t19197: F, t19204: F, t19226: F, t22004: F, t22008: F, t22114: F, t22327: F, t22349: F, t22355: F, t22361: F, t22369: F, t22375: F, t22387: F, t22394: F, t3487: F, t3593: F, t44691: F, t45329: F, t466: F, t4945: F, t498: F, t5011: F, t5075: F, t5080: F, t5086: F, t52479: F, t52480: F, t52485: F, t53545: F, t53613: F, t53646: F, t6150: F, t6168: F, t6238: F, t6244: F, t6252: F, t6253: F, t6257: F, t6260: F, t65262: F, t72577: F, t22334: F, t1251: F, t15820: F, t1761: F, t19209: F, t19214: F, t19220: F, t19232: F, t19234: F, t22007: F, t3598: F, t45350: F, t5055: F, t5060: F, t5088: F, t5089: F, t6267: F, t6268: F, t66822: F, t22337: F, t22328: F, t11606: F, t1720: F, t1760: F, t19208: F, t19249: F, t22113: F, t6243: F, t65208: F, t1256: F, t19267: F, t193: F, t27843: F, t336: F, t4700: F, t5091: F, t66897: F, t72104: F, t72106: F, t72138: F, t72201: F, t72203: F, t72207: F, t72209: F, t72211: F, t72213: F) -> F {
        let (t73535, t73541, t73571) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2632::<F>(t3447, t4904, t64779, t15402, t21749, t15376, t15382, t15390, t15395, t18543, t18546, t44635, t458, t4900, t4919, t4936, t52100, t52368, t6138, t64644, t64870, t65018, t65056, t71193, t72688);
        let t73575 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2633::<F>(t1090, t11569, t1174, t1177, t1178, t1184, t15293, t15376, t15390, t15395, t18321, t18404, t18416, t18417, t18420, t18421, t18431, t18559, t18564, t22038, t29614, t3447, t3449, t3451, t44487, t44504, t457, t460, t4889, t4900, t4901, t4908, t4913, t4919, t4928, t4934, t51971, t51981, t52096, t52110, t52124, t52281, t52288, t52355, t5971, t5975, t64667, t64686, t64696, t64699, t64702, t64811, t64858, t64878, t64881, t64885, t64994, t65002, t65008, t65023, t65087, t65128, t65136, t65139, t65142, t67060, t71133, t71138, t71158, t71164, t72788, t7319, t73192, t73199, t73201, t73252, t73272, t73274, t73276, t73279, t73316, t73330, t73355, t73369, t73399, t73439, t73444, t73451, t73480, t73491, t73496, t73525, t73535, t73541, t73571, t8034, t974);
        let (t73576, t73587) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2634::<F>(t225, t73575, t11665, t11668, t11678, t1215, t15659, t19083, t22162, t3577, t3578, t45296, t484, t488, t4965, t4974, t5012, t52893, t53516, t53520, t5975, t5979, t6164, t66646, t66648, t66668, t68, t73138, t73142);
        let t73592 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2635::<F>(t72180, t72233, t72268, t72299, t72333, t72357, t72380, t72405, t72452, t72484, t72522, t72552, t72593, t72622, t72654, t72683, t72712, t72735, t72783, t72823, t72842, t72878, t72911, t72938, t72970, t72996, t73019, t73048, t73078, t73108, t73126, t73587);
        let (t73613, t73630, t73663, t73670) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2636::<F>(t22398, t225, t1243, t72361, t1235, t22298, t11907, t11914, t11915, t1215, t15027, t15245, t19128, t19129, t19131, t19142, t19157, t19160, t22341, t22348, t22354, t22372, t22390, t3604, t3624, t44724, t44726, t5064, t53565);
        let (t73720, t73736) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2637::<F>(t5052, t6224, t11881, t1215, t1244, t1246, t19165, t19201, t22340, t22348, t22358, t22364, t22368, t22386, t3610, t3624, t3625, t44698, t44701, t44753, t44754, t45326, t491, t5068, t5072, t5084, t6218, t72217);
        let (t73755, t73789) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2638::<F>(t6218, t6739, t15032, t1756, t19154, t19170, t19189, t19190, t19203, t22354, t22389, t3610, t3624, t470, t493, t494, t5064, t5069, t5079, t53592, t6256, t6261, t65254, t66787, t73576, t73592);
        let t73844 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2639::<F>(t11881, t11883, t11904, t1215, t1235, t1244, t1246, t15245, t1755, t18940, t19146, t19201, t22243, t22348, t22365, t22389, t23508, t3610, t3612, t44785, t475, t4964, t5068, t5073, t5076, t52435, t6263, t6265, t73663);
        let t73852 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2640::<F>(t11881, t11883, t11888, t11889, t1190, t11904, t11907, t11914, t11915, t1201, t1215, t1238, t1241, t1244, t1246, t1247, t1249, t1252, t14972, t14980, t15027, t15032, t15245, t15659, t15700, t15797, t1729, t1734, t1751, t1758, t18572, t18940, t19120, t19123, t19138, t19139, t19153, t19156, t19166, t19174, t19176, t19180, t19197, t19204, t19226, t22004, t22008, t22114, t22327, t22340, t22349, t22355, t22361, t22364, t22368, t22369, t22375, t22386, t22387, t22394, t3487, t3593, t3604, t3610, t3612, t3624, t44691, t45329, t466, t475, t491, t4945, t498, t5011, t5052, t5064, t5075, t5079, t5080, t5086, t52479, t52480, t52485, t53545, t53613, t53646, t6150, t6168, t6238, t6244, t6252, t6253, t6256, t6257, t6260, t65262, t72577, t73575, t73592, t73613, t73630, t73663, t73670, t73720, t73736, t73755, t73789, t73844);
        let t73885 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2641::<F>(t22334, t225, t1238, t1251, t1252, t15797, t15820, t1761, t19209, t19214, t19220, t19232, t19234, t22007, t22008, t3593, t3598, t45350, t4945, t5055, t5060, t5088, t5089, t6244, t6267, t6268, t66822);
        let t73919 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2642::<F>(t22337, t225, t22328, t11606, t1235, t1238, t1252, t14980, t1720, t1760, t1761, t19120, t19208, t19214, t19220, t19226, t19232, t19249, t22113, t22394, t3487, t3598, t498, t5055, t5060, t5088, t5089, t6243, t6268, t65208);
        let t73931 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2643::<F>(t1256, t19267, t193, t27843, t336, t4700, t5091, t66897, t72104, t72106, t72138, t72201, t72203, t72207, t72209, t72211, t72213, t73852, t73885, t73919);
    t73931
}
