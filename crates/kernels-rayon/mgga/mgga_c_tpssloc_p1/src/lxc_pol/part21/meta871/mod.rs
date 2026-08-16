//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta871 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3200;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3201;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3202;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3203;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3204;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3205;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3206;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3207;
use chunk8::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3208;
use chunk9::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3209;
use chunk10::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3210;
use chunk11::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3211;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta871(t11697: f64, t18968: f64, t3577: f64, t11539: f64, t1174: f64, t18232: f64, t18215: f64, t1734: f64, t3584: f64, t375: f64, t11665: f64, t18371: f64, t15569: f64, t15572: f64, t15714: f64, t15740: f64, t15749: f64, t18364: f64, t3242: f64, t3508: f64, t3578: f64, t45250: f64, t4733: f64, t4950: f64, t5012: f64, t52615: f64, t53433: f64, t53440: f64, t53452: f64, t66372: f64, t66378: f64, t66380: f64, t1244: f64, t3068: f64, t478: f64, t6163: f64, t11734: f64, t1227: f64, t15498: f64, t15525: f64, t15541: f64, t19072: f64, t3515: f64, t3580: f64, t4582: f64, t4977: f64, t4989: f64, t5024: f64, t52919: f64, t53456: f64, t53468: f64, t53470: f64, t53476: f64, t53481: f64, t53490: f64, t53494: f64, t53496: f64, t53498: f64, t61855: f64, t18386: f64, t15608: f64, t225: f64, t65165: f64, t6183: f64, t698: f64, t1216: f64, t15470: f64, t15474: f64, t15700: f64, t18241: f64, t18383: f64, t18965: f64, t45119: f64, t45134: f64, t45266: f64, t45296: f64, t484: f64, t488: f64, t4954: f64, t52893: f64, t52897: f64, t53322: f64, t53515: f64, t53519: f64, t64874: f64, t68: f64, t65463: f64, t65518: f64, t65565: f64, t65610: f64, t65653: f64, t65685: f64, t65716: f64, t65764: f64, t65802: f64, t65835: f64, t65883: f64, t65925: f64, t65954: f64, t65990: f64, t66029: f64, t66067: f64, t66111: f64, t66157: f64, t66185: f64, t66219: f64, t66254: f64, t66282: f64, t66326: f64, t66353: f64, t66400: f64, t66442: f64, t66480: f64, t66528: f64, t66564: f64, t11871: f64, t11881: f64, t11888: f64, t1201: f64, t15022: f64, t15247: f64, t15426: f64, t1758: f64, t18301: f64, t19169: f64, t19174: f64, t19197: f64, t3507: f64, t3604: f64, t3610: f64, t3624: f64, t3625: f64, t44724: f64, t44726: f64, t44730: f64, t470: f64, t493: f64, t5011: f64, t5079: f64, t52479: f64, t52480: f64, t6252: f64, t6256: f64, t6260: f64, t65265: f64, t1246: f64, t15001: f64, t15019: f64, t15027: f64, t15032: f64, t15245: f64, t15253: f64, t15257: f64, t19129: f64, t19179: f64, t3590: f64, t44698: f64, t44701: f64, t44741: f64, t45320: f64, t4978: f64, t5068: f64, t5073: f64, t53613: f64, t53646: f64, t6218: f64, t6253: f64, t11914: f64, t14992: f64, t15429: f64, t15772: f64, t1729: f64, t19145: f64, t19153: f64, t19156: f64, t19170: f64, t3565: f64, t45323: f64, t4964: f64, t5076: f64, t5086: f64, t53545: f64, t6263: f64, t6265: f64, t1243: f64, t65955: f64, t11904: f64, t1247: f64, t1249: f64, t15000: f64, t15016: f64, t15241: f64, t1756: f64, t18572: f64, t19142: f64, t19157: f64, t19180: f64, t19203: f64, t23508: f64, t3612: f64, t3628: f64, t44691: f64, t44785: f64, t475: f64, t494: f64, t5064: f64, t5072: f64, t52447: f64, t6168: f64, t65347: f64, t19253: f64, t5088: f64, t11925: f64, t1238: f64, t1241: f64, t1251: f64, t1252: f64, t14980: f64, t15786: f64, t15803: f64, t15820: f64, t1760: f64, t1761: f64, t19208: f64, t19220: f64, t19234: f64, t3593: f64, t3598: f64, t3599: f64, t3631: f64, t45350: f64, t466: f64, t498: f64, t5055: f64, t5060: f64, t51925: f64, t51928: f64, t6243: f64, t6268: f64, t65208: f64, t65249: f64, t65343: f64, t65374: f64, t65408: f64, t19121: f64, t19259: f64, t11613: f64, t14972: f64, t15425: f64, t15787: f64, t15794: f64, t15797: f64, t1751: f64, t19209: f64, t19232: f64, t19249: f64, t3481: f64, t3487: f64, t3600: f64, t4940: f64, t4945: f64, t5052: f64, t5089: f64, t53658: f64, t6238: f64, t1254: f64, t1256: f64, t15834: f64, t193: f64, t336: f64, t4700: f64, t5095: f64, t63714: f64, t63717: f64, t63720: f64, t63722: f64, t63725: f64, t63729: f64, t64548: f64, t64558: f64, t64562: f64, t64564: f64, t64566: f64, t64602: f64, t65206: f64) -> f64 {
        let (t66566, t66571, t66575, t66583, t66597) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3200(t11697, t18968, t3577, t11539, t1174, t18232, t18215, t1734, t3584, t375, t11665, t18371);
        let t66601 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3201(t15569, t15572, t11665, t15714, t15740, t15749, t18364, t3242, t3508, t3577, t3578, t45250, t4733, t4950, t5012, t52615, t53433, t53440, t53452, t66372, t66378, t66380, t66566, t66571, t66575, t66583, t66597);
        let t66631 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3202(t1244, t3068, t478, t6163, t11734, t1227, t15498, t15525, t15541, t19072, t3515, t3580, t4582, t4977, t4989, t5024, t52919, t53456, t53468, t53470, t53476, t53481, t53490, t53494, t53496, t53498, t61855);
        let (t66662, t66670) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3203(t11697, t18386, t3577, t15608, t15740, t225, t65165, t1174, t6183, t698, t11665, t1216, t15470, t15474, t15569, t15700, t18241, t18383, t18965, t3578, t45119, t45134, t45266, t45296, t484, t488, t4954, t5012, t52893, t52897, t53322, t53515, t53519, t64874, t68);
        let t66675 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3204(t65463, t65518, t65565, t65610, t65653, t65685, t65716, t65764, t65802, t65835, t65883, t65925, t65954, t65990, t66029, t66067, t66111, t66157, t66185, t66219, t66254, t66282, t66326, t66353, t66400, t66442, t66480, t66528, t66564, t66601, t66631, t66670);
        let t66702 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3205(t11871, t11881, t11888, t1201, t15022, t15247, t15426, t1758, t18301, t19169, t19174, t19197, t3507, t3604, t3610, t3624, t3625, t44724, t44726, t44730, t470, t493, t5011, t5079, t52479, t52480, t6252, t6256, t6260, t65265, t66675);
        let t66737 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3206(t11871, t11888, t1244, t1246, t15001, t15019, t15027, t15032, t15245, t15253, t15257, t19129, t19179, t3507, t3590, t3604, t3610, t44698, t44701, t44741, t45320, t4978, t5011, t5068, t5073, t52480, t53613, t53646, t6218, t6252, t6253, t6256);
        let t66769 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3207(t11888, t11914, t1216, t14992, t15032, t15245, t15429, t15772, t1729, t19145, t19153, t19156, t19169, t19170, t3565, t3604, t3610, t45323, t4964, t5011, t5068, t5076, t5086, t52480, t53545, t6256, t6260, t6263, t6265);
        let t66802 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3208(t1243, t65955, t11881, t11904, t1247, t1249, t15000, t15016, t15241, t1756, t18572, t19142, t19157, t19180, t19203, t23508, t3507, t3604, t3610, t3612, t3628, t44691, t44785, t475, t494, t5064, t5072, t52447, t6168, t6252, t6256, t65347, t66662);
        let t66842 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3209(t19253, t225, t5088, t11925, t1238, t1241, t1251, t1252, t14980, t15786, t15803, t15820, t1760, t1761, t19208, t19220, t19234, t3593, t3598, t3599, t3631, t45350, t466, t498, t5055, t5060, t51925, t51928, t6243, t6268, t65208, t65249, t65343, t65374, t65408, t66675, t66702, t66737, t66769, t66802);
        let t66879 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3210(t19121, t225, t19259, t11613, t1252, t14972, t15425, t15787, t15794, t15797, t1751, t1761, t19209, t19220, t19232, t19234, t19249, t3481, t3487, t3600, t3631, t4940, t4945, t498, t5052, t5055, t5060, t5089, t53658, t6238, t6268);
        let t66885 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3211(t1254, t1256, t15834, t193, t336, t4700, t5095, t63714, t63717, t63720, t63722, t63725, t63729, t64548, t64558, t64562, t64564, t64566, t64602, t65206, t66842, t66879);
    t66885
}
