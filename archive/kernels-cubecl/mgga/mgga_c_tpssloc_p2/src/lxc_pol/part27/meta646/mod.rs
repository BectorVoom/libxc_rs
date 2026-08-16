//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta646 (260520-c91 hierarchical CSE).
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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2219;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2220;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2221;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2222;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2223;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2224;
use chunk6::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2225;
use chunk7::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2226;
use chunk8::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2227;
use chunk9::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2228;
use chunk10::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2229;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta646<F: Float>(t14192: F, t6717: F, t13965: F, t6755: F, t25577: F, t3103: F, t25650: F, t3030: F, t82890: F, t1618: F, t23422: F, t23433: F, t23489: F, t23544: F, t25652: F, t25654: F, t25655: F, t25679: F, t3123: F, t3128: F, t4585: F, t4609: F, t4649: F, t4652: F, t7583: F, t82981: F, t83068: F, t83127: F, t1933: F, t23479: F, t88405: F, t1409: F, t1937: F, t6722: F, t1015: F, t10475: F, t13762: F, t14041: F, t1615: F, t23419: F, t23678: F, t25653: F, t25658: F, t25660: F, t25661: F, t3040: F, t3120: F, t360: F, t4575: F, t4579: F, t82516: F, t82542: F, t82754: F, t83008: F, t83134: F, t88537: F, t14501: F, t23472: F, t25678: F, t14198: F, t4590: F, t4596: F, t4600: F, t82848: F, t82956: F, t83139: F, t83153: F, t83157: F, t83159: F, t83165: F, t83167: F, t83172: F, t83206: F, t88254: F, t88275: F, t88303: F, t88327: F, t88358: F, t88397: F, t88437: F, t88472: F, t88504: F, t88533: F, t88570: F, t88597: F, t88632: F, t7554: F, t82632: F, t14529: F, t14545: F, t23327: F, t23341: F, t23346: F, t23395: F, t25406: F, t25413: F, t25732: F, t25784: F, t3016: F, t3026: F, t349: F, t388: F, t4660: F, t6687: F, t6816: F, t7553: F, t7565: F, t82437: F, t82463: F, t82490: F, t83296: F, t83303: F, t225: F, t25820: F, t23384: F, t25827: F, t25436: F, t23328: F, t23394: F, t10170: F, t1049: F, t1050: F, t1066: F, t13735: F, t13743: F, t14549: F, t14659: F, t1634: F, t1635: F, t1956: F, t23331: F, t254: F, t25712: F, t25759: F, t343: F, t50703: F, t6690: F, t6704: F, t6771: F, t7625: F, t82481: F, t83276: F, t83281: F, t883: F, t1054: F, t4693: F, t13783: F, t1926: F, t221: F, t25432: F, t10164: F, t1052: F, t1065: F, t14658: F, t1955: F, t23329: F, t23330: F, t23369: F, t23402: F, t23581: F, t25429: F, t25705: F, t25749: F, t25757: F, t25801: F, t25810: F, t2771: F, t2780: F, t3174: F, t3966: F, t4664: F, t4694: F, t6815: F, t7600: F, t82382: F, t83285: F, t83287: F, t884: F, t990: F, t25806: F, t6680: F, t43603: F, t10160: F, t14548: F, t23336: F, t25420: F, t25739: F, t25758: F, t25778: F, t3010: F, t3169: F, t3176: F, t3206: F, t4542: F, t6699: F, t7561: F, t83316: F, t83318: F, t991: F, t4657: F, t6688: F, t7566: F, t25400: F, t13611: F, t13933: F, t13939: F, t14552: F, t1922: F, t1945: F, t23323: F, t23372: F, t23725: F, t25755: F, t4557: F, t6689: F, t6691: F, t6776: F, t7562: F, t83329: F, t25416: F, t82431: F, t1921: F, t25811: F, t14526: F, t1920: F, t1927: F, t25453: F, t25738: F, t25816: F, t2776: F, t345: F, t387: F, t4552: F, t6768: F, t82357: F, t82402: F, t83342: F, t83344: F, t986: F, t1598: F, t3008: F, t25407: F, t25513: F, t25726: F, t14165: F, t14626: F, t23601: F, t23603: F, t23604: F, t23613: F, t23670: F, t23677: F, t25471: F, t25475: F, t25503: F, t25510: F, t25545: F, t25721: F, t7603: F, t82750: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t88655, t88662) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2219::<F>(t14192, t6717, t13965, t6755, t25577, t3103, t25650, t3030, t82890, t1618, t23422, t23433, t23489, t23544, t25652, t25654, t25655, t25679, t3123, t3128, t4585, t4609, t4649, t4652, t7583, t82981, t83068, t83127);
        let t88702 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2220::<F>(t1933, t23479, t88405, t1409, t1937, t6722, t1015, t10475, t13762, t14041, t1615, t23419, t23678, t25652, t25653, t25658, t25660, t25661, t3040, t3120, t360, t4575, t4579, t4649, t82516, t82542, t82754, t83008, t83134, t88537, t88655);
        let t88724 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2221::<F>(t14501, t23419, t1015, t23472, t25678, t14198, t23544, t4590, t4596, t4600, t6717, t82848, t82956, t83139, t83153, t83157, t83159, t83165, t83167, t83172, t83206);
        let t88728 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2222::<F>(t88254, t88275, t88303, t88327, t88358, t88397, t88437, t88472, t88504, t88533, t88570, t88597, t88632, t88662, t88702, t88724);
        let t88742 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2223::<F>(t7554, t82632, t14529, t14545, t23327, t23341, t23346, t23395, t25406, t25413, t25732, t25784, t3016, t3026, t349, t388, t4660, t6687, t6816, t7553, t7565, t82437, t82463, t82490, t83296, t83303, t88728);
        let t88779 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2224::<F>(t225, t25820, t23384, t25827, t25436, t23328, t23394, t10170, t1049, t1050, t1066, t13735, t13743, t14549, t14659, t1634, t1635, t1956, t23327, t23331, t254, t25712, t25759, t343, t50703, t6687, t6690, t6704, t6771, t7625, t82481, t83276, t83281, t883);
        let (t88804, t88810, t88827) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2225::<F>(t1054, t4693, t13783, t1926, t221, t25432, t10164, t10170, t1052, t1065, t14658, t1955, t23327, t23329, t23330, t23369, t23402, t23581, t25429, t25705, t25749, t25757, t25801, t25810, t2771, t2780, t3174, t388, t3966, t4664, t4694, t6687, t6815, t7554, t7600, t82382, t83285, t83287, t884, t990);
        let t88867 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2226::<F>(t25806, t6680, t1955, t43603, t10160, t13735, t1409, t14548, t23327, t23329, t23330, t23336, t23346, t254, t25420, t25739, t25757, t25758, t25759, t25778, t25801, t3010, t3169, t3176, t3206, t4542, t6687, t6699, t7561, t7625, t83316, t83318, t991);
        let t88900 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2227::<F>(t4657, t6688, t7566, t82632, t23384, t25400, t13611, t13933, t13939, t14552, t1922, t1945, t23323, t23346, t23372, t23725, t25420, t25755, t25827, t3026, t3176, t388, t4557, t4694, t6687, t6689, t6690, t6691, t6776, t7562, t83329);
        let t88940 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2228::<F>(t25416, t82431, t1921, t88804, t23384, t25811, t1052, t14526, t1920, t1927, t225, t23327, t23329, t23336, t23725, t25453, t25738, t25749, t25816, t2776, t3026, t3174, t345, t387, t388, t4552, t4660, t4693, t6687, t6768, t6815, t7553, t82357, t82402, t83342, t83344, t986);
        let (t88941, t88954, t89001) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2229::<F>(t1598, t3008, t23384, t25407, t25513, t82431, t25726, t14165, t14626, t23327, t23601, t23603, t23604, t23613, t23670, t23677, t23678, t25471, t25475, t25503, t25510, t25545, t25721, t7603, t82402, t82750);
    (t88728, t88742, t88779, t88810, t88827, t88867, t88900, t88940, t88941, t88954, t89001)
}
