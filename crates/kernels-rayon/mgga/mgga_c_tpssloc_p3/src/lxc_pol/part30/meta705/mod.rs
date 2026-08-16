//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta705 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2308;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2309;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2310;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2311;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2312;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2313;
use chunk6::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2314;
use chunk7::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2315;
use chunk8::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2316;
use chunk9::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2317;
use chunk10::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2318;
use chunk11::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2319;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta705(t5932: f64, t6743: f64, t28653: f64, t82822: f64, t1014: f64, t1058: f64, t1060: f64, t11046: f64, t14608: f64, t1625: f64, t17959: f64, t18093: f64, t1945: f64, t23478: f64, t23601: f64, t23602: f64, t23633: f64, t25492: f64, t25516: f64, t25554: f64, t25558: f64, t25712: f64, t28596: f64, t28601: f64, t28641: f64, t3186: f64, t4673: f64, t6687: f64, t82717: f64, t89175: f64, t89224: f64, t5936: f64, t1022: f64, t5392: f64, t6800: f64, t23518: f64, t5928: f64, t17843: f64, t1949: f64, t23346: f64, t23604: f64, t28602: f64, t28610: f64, t3180: f64, t5844: f64, t6805: f64, t83239: f64, t83240: f64, t83245: f64, t884: f64, t89256: f64, t89292: f64, t89294: f64, t89296: f64, t23384: f64, t28657: f64, t1615: f64, t18107: f64, t23327: f64, t23613: f64, t25429: f64, t25510: f64, t25549: f64, t25705: f64, t25713: f64, t2770: f64, t2775: f64, t28609: f64, t28614: f64, t3200: f64, t3961: f64, t7619: f64, t883: f64, t89309: f64, t89310: f64, t89327: f64, t99180: f64, t1003: f64, t17187: f64, t18086: f64, t23635: f64, t25500: f64, t28634: f64, t28660: f64, t353: f64, t383: f64, t4542: f64, t4669: f64, t5398: f64, t6784: f64, t6785: f64, t6811: f64, t7614: f64, t82668: f64, t83233: f64, t89329: f64, t99859: f64, t1920: f64, t28630: f64, t968: f64, t5872: f64, t6768: f64, t83244: f64, t89242: f64, t18047: f64, t1948: f64, t28663: f64, t3201: f64, t345: f64, t4649: f64, t5838: f64, t7593: f64, t89360: f64, t89362: f64, t89366: f64, t89369: f64, t89505: f64, t986: f64, t17686: f64, t17691: f64, t18138: f64, t23670: f64, t25721: f64, t28593: f64, t28618: f64, t28622: f64, t28637: f64, t28652: f64, t3966: f64, t82625: f64, t82799: f64, t88022: f64, t89071: f64, t89176: f64, t28671: f64, t82736: f64, t14651: f64, t1599: f64, t25479: f64, t25535: f64, t3188: f64, t7620: f64, t82809: f64, t89243: f64, t89421: f64, t89429: f64, t89431: f64, t89445: f64, t89501: f64, t28557: f64, t11034: f64, t1409: f64, t1539: f64, t23685: f64, t25497: f64, t28642: f64, t28674: f64, t4684: f64, t5681: f64, t5866: f64, t6797: f64, t6801: f64, t82830: f64, t89235: f64, t89449: f64, t362: f64, t5914: f64, t14618: f64, t25568: f64, t25708: f64, t28605: f64, t28631: f64, t5685: f64, t5903: f64, t6680: f64, t6813: f64, t7603: f64, t89532: f64, t89546: f64, t99921: f64, t100025: f64, t100068: f64, t100103: f64, t100147: f64, t100176: f64, t100195: f64, t1052: f64, t1055: f64, t1603: f64, t17875: f64, t18070: f64, t18074: f64, t18166: f64, t23581: f64, t25755: f64, t25757: f64, t28499: f64, t28679: f64, t3169: f64, t388: f64, t4694: f64, t5848: f64, t6699: f64, t6771: f64, t6816: f64, t83459: f64, t88851: f64, t89662: f64, t89672: f64, t99983: f64, t28719: f64, t3216: f64, t1068: f64, t1070: f64, t1637: f64, t18169: f64, t193: f64, t23738: f64, t23742: f64, t25840: f64, t25845: f64, t336: f64, t4696: f64, t4700: f64, t5946: f64, t5950: f64, t6822: f64, t83472: f64, t83479: f64, t89698: f64, t89702: f64, t99104: f64, t99143: f64, t99172: f64, t99202: f64, t99238: f64, t99271: f64, t99313: f64, t99353: f64, t99390: f64, t99422: f64, t99450: f64, t99866: f64, t99894: f64, t99930: f64, t99959: f64, t25365: f64, t57911: f64, t10143: f64, t1484: f64, t25374: f64, t16596: f64, t16944: f64, t16949: f64, t1877: f64, t1915: f64, t202: f64, t22959: f64, t23290: f64, t23295: f64, t25013: f64, t2522: f64, t25354: f64, t25358: f64, t28248: f64, t4255: f64, t4314: f64, t5544: f64, t6666: f64, t6670: f64, t67128: f64, t7541: f64, t82312: f64, t870: f64, t97999: f64, t98003: f64, t98007: f64, t98011: f64, t99042: f64) -> (f64, f64) {
        let t100225 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2308(t5932, t6743, t28653, t82822, t1014, t1058, t1060, t11046, t14608, t1625, t17959, t18093, t1945, t23478, t23601, t23602, t23633, t25492, t25516, t25554, t25558, t25712, t28596, t28601, t28641, t3186, t4673, t6687, t82717, t89175, t89224);
        let (t100236, t100253) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2309(t5936, t6743, t1022, t5392, t6800, t23518, t5928, t17843, t1949, t23346, t23604, t23633, t25554, t28602, t28610, t3180, t5844, t6687, t6805, t83239, t83240, t83245, t884, t89256, t89292, t89294, t89296);
        let t100287 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2310(t23384, t28657, t1058, t1060, t1615, t1625, t18107, t23327, t23346, t23613, t23633, t25429, t25510, t25549, t25705, t25713, t2770, t2775, t28609, t28614, t3200, t3961, t6687, t6743, t7619, t883, t89309, t89310, t89327, t99180);
        let t100314 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2311(t100236, t1003, t1022, t17187, t18086, t23346, t23633, t23635, t25500, t28634, t28653, t28660, t353, t383, t4542, t4669, t5398, t6687, t6784, t6785, t6800, t6811, t7614, t82668, t83233, t89329, t99859);
        let (t100326, t100334, t100341) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2312(t1920, t28630, t968, t5872, t6768, t83244, t89242, t1058, t1060, t18047, t1948, t23346, t28663, t3200, t3201, t345, t4649, t5838, t6687, t6805, t7593, t89360, t89362, t89366, t89369, t89505, t986);
        let t100377 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2313(t1022, t1058, t1060, t1615, t17686, t17691, t18138, t23346, t23613, t23633, t23635, t23670, t25429, t25510, t25721, t28593, t28618, t28622, t28637, t28652, t3186, t3966, t6800, t7619, t82625, t82799, t88022, t89071, t89176);
        let t100396 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2314(t23384, t28618, t28671, t82736, t100326, t100334, t14651, t1599, t25479, t25535, t3186, t3188, t6687, t7620, t82809, t89243, t89421, t89429, t89431, t89445, t89501);
        let t100430 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2315(t23384, t28610, t28557, t6743, t1058, t1060, t11034, t1409, t1539, t23633, t23635, t23685, t25497, t28601, t28642, t28674, t3180, t3200, t4649, t4669, t4684, t5681, t5866, t6687, t6768, t6784, t6797, t6800, t6801, t82830, t89235, t89449);
        let t100459 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2316(t23384, t28660, t28614, t362, t5914, t14618, t23327, t23670, t23685, t25568, t25708, t25713, t28605, t28631, t4669, t5685, t5903, t6680, t6687, t6784, t6813, t7603, t884, t89532, t89546, t99921);
        let t100489 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2317(t100025, t100068, t100103, t100147, t100176, t100195, t100225, t100253, t100287, t100314, t100341, t100377, t100396, t100430, t100459, t1052, t1055, t1603, t17875, t18070, t18074, t18166, t1945, t23581, t25705, t25755, t25757, t28499, t28679, t3169, t388, t4694, t5838, t5848, t6687, t6699, t6768, t6771, t6816, t83459, t88851, t89662, t89672, t99983);
        let t100528 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2318(t28719, t3216, t100489, t1068, t1070, t1637, t18169, t193, t23738, t23742, t25840, t25845, t336, t4696, t4700, t5946, t5950, t6822, t83472, t83479, t89698, t89702, t99104, t99143, t99172, t99202, t99238, t99271, t99313, t99353, t99390, t99422, t99450, t99866, t99894, t99930, t99959);
        let t100578 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2319(t25365, t57911, t10143, t1484, t25374, t16596, t16944, t16949, t1877, t1915, t193, t202, t22959, t23290, t23295, t25013, t2522, t25354, t25358, t28248, t4255, t4314, t5544, t6666, t6670, t67128, t7541, t82312, t870, t97999, t98003, t98007, t98011, t99042);
    (t100528, t100578)
}
