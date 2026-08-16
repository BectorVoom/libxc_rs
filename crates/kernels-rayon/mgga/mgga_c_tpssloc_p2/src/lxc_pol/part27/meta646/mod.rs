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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta646(t14192: f64, t6717: f64, t13965: f64, t6755: f64, t25577: f64, t3103: f64, t25650: f64, t3030: f64, t82890: f64, t1618: f64, t23422: f64, t23433: f64, t23489: f64, t23544: f64, t25652: f64, t25654: f64, t25655: f64, t25679: f64, t3123: f64, t3128: f64, t4585: f64, t4609: f64, t4649: f64, t4652: f64, t7583: f64, t82981: f64, t83068: f64, t83127: f64, t1933: f64, t23479: f64, t88405: f64, t1409: f64, t1937: f64, t6722: f64, t1015: f64, t10475: f64, t13762: f64, t14041: f64, t1615: f64, t23419: f64, t23678: f64, t25653: f64, t25658: f64, t25660: f64, t25661: f64, t3040: f64, t3120: f64, t360: f64, t4575: f64, t4579: f64, t82516: f64, t82542: f64, t82754: f64, t83008: f64, t83134: f64, t88537: f64, t14501: f64, t23472: f64, t25678: f64, t14198: f64, t4590: f64, t4596: f64, t4600: f64, t82848: f64, t82956: f64, t83139: f64, t83153: f64, t83157: f64, t83159: f64, t83165: f64, t83167: f64, t83172: f64, t83206: f64, t88254: f64, t88275: f64, t88303: f64, t88327: f64, t88358: f64, t88397: f64, t88437: f64, t88472: f64, t88504: f64, t88533: f64, t88570: f64, t88597: f64, t88632: f64, t7554: f64, t82632: f64, t14529: f64, t14545: f64, t23327: f64, t23341: f64, t23346: f64, t23395: f64, t25406: f64, t25413: f64, t25732: f64, t25784: f64, t3016: f64, t3026: f64, t349: f64, t388: f64, t4660: f64, t6687: f64, t6816: f64, t7553: f64, t7565: f64, t82437: f64, t82463: f64, t82490: f64, t83296: f64, t83303: f64, t225: f64, t25820: f64, t23384: f64, t25827: f64, t25436: f64, t23328: f64, t23394: f64, t10170: f64, t1049: f64, t1050: f64, t1066: f64, t13735: f64, t13743: f64, t14549: f64, t14659: f64, t1634: f64, t1635: f64, t1956: f64, t23331: f64, t254: f64, t25712: f64, t25759: f64, t343: f64, t50703: f64, t6690: f64, t6704: f64, t6771: f64, t7625: f64, t82481: f64, t83276: f64, t83281: f64, t883: f64, t1054: f64, t4693: f64, t13783: f64, t1926: f64, t221: f64, t25432: f64, t10164: f64, t1052: f64, t1065: f64, t14658: f64, t1955: f64, t23329: f64, t23330: f64, t23369: f64, t23402: f64, t23581: f64, t25429: f64, t25705: f64, t25749: f64, t25757: f64, t25801: f64, t25810: f64, t2771: f64, t2780: f64, t3174: f64, t3966: f64, t4664: f64, t4694: f64, t6815: f64, t7600: f64, t82382: f64, t83285: f64, t83287: f64, t884: f64, t990: f64, t25806: f64, t6680: f64, t43603: f64, t10160: f64, t14548: f64, t23336: f64, t25420: f64, t25739: f64, t25758: f64, t25778: f64, t3010: f64, t3169: f64, t3176: f64, t3206: f64, t4542: f64, t6699: f64, t7561: f64, t83316: f64, t83318: f64, t991: f64, t4657: f64, t6688: f64, t7566: f64, t25400: f64, t13611: f64, t13933: f64, t13939: f64, t14552: f64, t1922: f64, t1945: f64, t23323: f64, t23372: f64, t23725: f64, t25755: f64, t4557: f64, t6689: f64, t6691: f64, t6776: f64, t7562: f64, t83329: f64, t25416: f64, t82431: f64, t1921: f64, t25811: f64, t14526: f64, t1920: f64, t1927: f64, t25453: f64, t25738: f64, t25816: f64, t2776: f64, t345: f64, t387: f64, t4552: f64, t6768: f64, t82357: f64, t82402: f64, t83342: f64, t83344: f64, t986: f64, t1598: f64, t3008: f64, t25407: f64, t25513: f64, t25726: f64, t14165: f64, t14626: f64, t23601: f64, t23603: f64, t23604: f64, t23613: f64, t23670: f64, t23677: f64, t25471: f64, t25475: f64, t25503: f64, t25510: f64, t25545: f64, t25721: f64, t7603: f64, t82750: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t88655, t88662) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2219(t14192, t6717, t13965, t6755, t25577, t3103, t25650, t3030, t82890, t1618, t23422, t23433, t23489, t23544, t25652, t25654, t25655, t25679, t3123, t3128, t4585, t4609, t4649, t4652, t7583, t82981, t83068, t83127);
        let t88702 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2220(t1933, t23479, t88405, t1409, t1937, t6722, t1015, t10475, t13762, t14041, t1615, t23419, t23678, t25652, t25653, t25658, t25660, t25661, t3040, t3120, t360, t4575, t4579, t4649, t82516, t82542, t82754, t83008, t83134, t88537, t88655);
        let t88724 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2221(t14501, t23419, t1015, t23472, t25678, t14198, t23544, t4590, t4596, t4600, t6717, t82848, t82956, t83139, t83153, t83157, t83159, t83165, t83167, t83172, t83206);
        let t88728 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2222(t88254, t88275, t88303, t88327, t88358, t88397, t88437, t88472, t88504, t88533, t88570, t88597, t88632, t88662, t88702, t88724);
        let t88742 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2223(t7554, t82632, t14529, t14545, t23327, t23341, t23346, t23395, t25406, t25413, t25732, t25784, t3016, t3026, t349, t388, t4660, t6687, t6816, t7553, t7565, t82437, t82463, t82490, t83296, t83303, t88728);
        let t88779 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2224(t225, t25820, t23384, t25827, t25436, t23328, t23394, t10170, t1049, t1050, t1066, t13735, t13743, t14549, t14659, t1634, t1635, t1956, t23327, t23331, t254, t25712, t25759, t343, t50703, t6687, t6690, t6704, t6771, t7625, t82481, t83276, t83281, t883);
        let (t88804, t88810, t88827) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2225(t1054, t4693, t13783, t1926, t221, t25432, t10164, t10170, t1052, t1065, t14658, t1955, t23327, t23329, t23330, t23369, t23402, t23581, t25429, t25705, t25749, t25757, t25801, t25810, t2771, t2780, t3174, t388, t3966, t4664, t4694, t6687, t6815, t7554, t7600, t82382, t83285, t83287, t884, t990);
        let t88867 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2226(t25806, t6680, t1955, t43603, t10160, t13735, t1409, t14548, t23327, t23329, t23330, t23336, t23346, t254, t25420, t25739, t25757, t25758, t25759, t25778, t25801, t3010, t3169, t3176, t3206, t4542, t6687, t6699, t7561, t7625, t83316, t83318, t991);
        let t88900 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2227(t4657, t6688, t7566, t82632, t23384, t25400, t13611, t13933, t13939, t14552, t1922, t1945, t23323, t23346, t23372, t23725, t25420, t25755, t25827, t3026, t3176, t388, t4557, t4694, t6687, t6689, t6690, t6691, t6776, t7562, t83329);
        let t88940 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2228(t25416, t82431, t1921, t88804, t23384, t25811, t1052, t14526, t1920, t1927, t225, t23327, t23329, t23336, t23725, t25453, t25738, t25749, t25816, t2776, t3026, t3174, t345, t387, t388, t4552, t4660, t4693, t6687, t6768, t6815, t7553, t82357, t82402, t83342, t83344, t986);
        let (t88941, t88954, t89001) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2229(t1598, t3008, t23384, t25407, t25513, t82431, t25726, t14165, t14626, t23327, t23601, t23603, t23604, t23613, t23670, t23677, t23678, t25471, t25475, t25503, t25510, t25545, t25721, t7603, t82402, t82750);
    (t88728, t88742, t88779, t88810, t88827, t88867, t88900, t88940, t88941, t88954, t89001)
}
