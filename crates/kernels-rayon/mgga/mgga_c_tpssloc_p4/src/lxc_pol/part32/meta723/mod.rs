//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta723 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2306;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2307;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2308;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2309;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2310;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2311;
use chunk6::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2312;
use chunk7::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2313;
use chunk8::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2314;
use chunk9::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2315;
use chunk10::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2316;
use chunk11::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2317;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta723(t24667: f64, t6252: f64, t1653: f64, t8039: f64, t85822: f64, t6224: f64, t7348: f64, t24574: f64, t29741: f64, t29614: f64, t7327: f64, t103683: f64, t24589: f64, t24833: f64, t24858: f64, t27507: f64, t27520: f64, t27536: f64, t27537: f64, t27562: f64, t29781: f64, t3624: f64, t3625: f64, t5975: f64, t7283: f64, t7362: f64, t7373: f64, t7377: f64, t8066: f64, t8073: f64, t85820: f64, t86037: f64, t86102: f64, t94966: f64, t95803: f64, t95813: f64, t29702: f64, t103515: f64, t11907: f64, t1216: f64, t1716: f64, t18525: f64, t18946: f64, t19203: f64, t2148: f64, t24812: f64, t27489: f64, t27490: f64, t27492: f64, t27496: f64, t27510: f64, t27540: f64, t27732: f64, t29709: f64, t3610: f64, t6140: f64, t7381: f64, t8082: f64, t94858: f64, t95033: f64, t6260: f64, t24660: f64, t1215: f64, t5392: f64, t7376: f64, t27736: f64, t7999: f64, t103218: f64, t11904: f64, t24849: f64, t27406: f64, t27455: f64, t27525: f64, t27532: f64, t27733: f64, t29678: f64, t29719: f64, t29723: f64, t4930: f64, t5068: f64, t7365: f64, t7382: f64, t8077: f64, t86039: f64, t86076: f64, t86077: f64, t94837: f64, t95048: f64, t24826: f64, t29716: f64, t103615: f64, t24745: f64, t27453: f64, t27460: f64, t27481: f64, t27484: f64, t27498: f64, t3612: f64, t7368: f64, t85918: f64, t85941: f64, t85952: f64, t85963: f64, t94874: f64, t95069: f64, t8070: f64, t94490: f64, t86036: f64, t95760: f64, t1409: f64, t1734: f64, t19138: f64, t24851: f64, t27502: f64, t29735: f64, t3966: f64, t5011: f64, t6256: f64, t86015: f64, t86116: f64, t95098: f64, t95114: f64, t95197: f64, t95201: f64, t95761: f64, t8074: f64, t94909: f64, t29745: f64, t29705: f64, t477: f64, t6238: f64, t1090: f64, t17635: f64, t19145: f64, t24820: f64, t24821: f64, t27549: f64, t27550: f64, t27551: f64, t29753: f64, t85863: f64, t85986: f64, t86000: f64, t95125: f64, t95134: f64, t95136: f64, t1186: f64, t11881: f64, t1751: f64, t19165: f64, t24814: f64, t24815: f64, t27517: f64, t27533: f64, t29708: f64, t29711: f64, t29726: f64, t3242: f64, t3961: f64, t5079: f64, t6146: f64, t94395: f64, t95092: f64, t95163: f64, t95165: f64, t95192: f64, t95213: f64, t29777: f64, t7359: f64, t29759: f64, t1244: f64, t1246: f64, t15245: f64, t19120: f64, t19169: f64, t2121: f64, t2147: f64, t24776: f64, t27546: f64, t27574: f64, t27721: f64, t462: f64, t5971: f64, t7375: f64, t95714: f64, t95722: f64, t29790: f64, t29763: f64, t8067: f64, t11914: f64, t1201: f64, t18572: f64, t18940: f64, t19153: f64, t2144: f64, t2152: f64, t27466: f64, t27474: f64, t27478: f64, t29773: f64, t4733: f64, t5064: f64, t8054: f64, t95726: f64, t11888: f64, t15032: f64, t1729: f64, t19156: f64, t19179: f64, t27465: f64, t27516: f64, t27722: f64, t29664: f64, t29712: f64, t3604: f64, t4964: f64, t6168: f64, t7389: f64, t8083: f64, t8085: f64, t95747: f64, t95751: f64, t95758: f64, t27604: f64, t4993: f64, t19095: f64, t24733: f64, t1207: f64, t19024: f64, t7337: f64, t19046: f64, t7338: f64, t6169: f64, t7344: f64, t1218: f64, t1232: f64, t1737: f64, t1748: f64, t18307: f64, t18943: f64, t18959: f64, t24716: f64, t6221: f64, t7339: f64, t7345: f64, t86164: f64, t95242: f64, t95244: f64, t95276: f64, t95440: f64, t18375: f64, t27599: f64, t4997: f64, t18360: f64, t18364: f64, t18397: f64, t18401: f64, t19002: f64, t19016: f64, t24741: f64, t27617: f64, t4950: f64, t4980: f64, t4984: f64, t5014: f64, t5030: f64, t86324: f64, t86327: f64, t95566: f64, t95623: f64, t95627: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t103707, t103733) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2306(t24667, t6252, t1653, t8039, t85822, t6224, t7348, t24574, t29741, t29614, t7327, t103683, t24589, t24833, t24858, t27507, t27520, t27536, t27537, t27562, t29781, t3624, t3625, t5975, t7283, t7362, t7373, t7377, t8066, t8073, t85820, t86037, t86102, t94966, t95803, t95813);
        let t103766 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2307(t24574, t29702, t103515, t11907, t1216, t1716, t18525, t18946, t19203, t2148, t24812, t27489, t27490, t27492, t27496, t27507, t27510, t27536, t27540, t27732, t29709, t3610, t6140, t7283, t7373, t7381, t8082, t94858, t95033);
        let (t103779, t103801) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2308(t6260, t7327, t24660, t6252, t1215, t5392, t7376, t27736, t7999, t103218, t11904, t24849, t27406, t27455, t27525, t27532, t27733, t29678, t29719, t29723, t3610, t4930, t5068, t7283, t7365, t7382, t8077, t86037, t86039, t86076, t86077, t94837, t95048);
        let t103829 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2309(t24826, t29716, t103218, t103615, t103707, t1216, t24745, t27406, t27453, t27460, t27481, t27484, t27498, t3610, t3612, t7283, t7368, t85918, t85941, t85952, t85963, t94858, t94874, t95069);
        let t103864 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2310(t8070, t94490, t86036, t95760, t103779, t1409, t1734, t19138, t24849, t24851, t27502, t27507, t27532, t27540, t29735, t3624, t3966, t5011, t6256, t7327, t7376, t8082, t86015, t86116, t95098, t95114, t95197, t95201, t95761);
        let t103889 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2311(t8074, t94909, t24826, t29745, t24574, t29705, t477, t6238, t1090, t17635, t19145, t24812, t24820, t24821, t27549, t27550, t27551, t29753, t7283, t7362, t85863, t85986, t86000, t95125, t95134, t95136);
        let t103918 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2312(t1186, t11881, t1751, t19145, t19165, t24812, t24814, t24815, t27517, t27533, t27549, t27550, t29708, t29711, t29719, t29726, t3242, t3610, t3624, t3961, t5068, t5079, t6146, t7283, t7381, t94395, t95092, t95163, t95165, t95192, t95213);
        let t103949 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2313(t24574, t29777, t29678, t7359, t29759, t1244, t1246, t15245, t1734, t19120, t19169, t2121, t2147, t24776, t24858, t27406, t27546, t27574, t27721, t29711, t3624, t462, t5079, t5971, t7283, t7373, t7375, t7376, t95714, t95722);
        let t103978 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2314(t24574, t29790, t29763, t8067, t94490, t11914, t1201, t1244, t1246, t18572, t18940, t19153, t2144, t2152, t27406, t27460, t27466, t27474, t27478, t29708, t29773, t4733, t5011, t5064, t7283, t7362, t8054, t95726);
        let t104002 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2315(t11888, t1215, t1244, t1246, t15032, t1729, t19156, t19179, t24589, t27465, t27516, t27722, t29664, t29708, t29712, t3604, t4964, t6168, t7373, t7375, t7376, t7389, t8083, t8085, t95747, t95751, t95758);
        let t104029 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2316(t27604, t4993, t19095, t24733, t1207, t19024, t7337, t19046, t7338, t6169, t7344, t1218, t1232, t1737, t1748, t18307, t18943, t18959, t24716, t6221, t7339, t7345, t86164, t95242, t95244, t95276, t95440);
        let t104056 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2317(t18375, t7339, t27599, t4997, t18360, t18364, t18397, t18401, t19002, t19016, t24741, t27617, t4950, t4980, t4984, t5014, t5030, t86324, t86327, t95566, t95623, t95627);
    (t103733, t103766, t103801, t103829, t103864, t103889, t103918, t103949, t103978, t104002, t104029, t104056)
}
