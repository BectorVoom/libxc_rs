//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta647 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2230;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2231;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2232;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2233;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2234;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2235;
use chunk6::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2236;
use chunk7::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2237;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta647(t25608: f64, t6743: f64, t1948: f64, t6733: f64, t23631: f64, t61066: f64, t974: f64, t12652: f64, t14586: f64, t14595: f64, t23323: f64, t23327: f64, t23609: f64, t23657: f64, t23673: f64, t25502: f64, t25510: f64, t25511: f64, t25512: f64, t25523: f64, t6797: f64, t6799: f64, t6800: f64, t6801: f64, t7603: f64, t7615: f64, t82539: f64, t82555: f64, t82643: f64, t82657: f64, t23511: f64, t7577: f64, t23665: f64, t25524: f64, t23384: f64, t25518: f64, t13611: f64, t23346: f64, t23601: f64, t23670: f64, t23679: f64, t25476: f64, t6687: f64, t6784: f64, t6785: f64, t82562: f64, t82564: f64, t82574: f64, t82576: f64, t82590: f64, t82605: f64, t10277: f64, t381: f64, t225: f64, t25714: f64, t12648: f64, t14165: f64, t14644: f64, t23613: f64, t23686: f64, t25429: f64, t25456: f64, t25470: f64, t25517: f64, t3010: f64, t6786: f64, t7614: f64, t82618: f64, t82629: f64, t82633: f64, t82635: f64, t7604: f64, t82573: f64, t3961: f64, t6746: f64, t11046: f64, t1409: f64, t14213: f64, t14571: f64, t14630: f64, t1629: f64, t23633: f64, t23635: f64, t23678: f64, t23685: f64, t25540: f64, t25544: f64, t25717: f64, t25722: f64, t3120: f64, t4347: f64, t7619: f64, t82661: f64, t83239: f64, t83240: f64, t83245: f64, t25718: f64, t25541: f64, t25545: f64, t25503: f64, t10216: f64, t1049: f64, t14605: f64, t23692: f64, t23697: f64, t25497: f64, t25500: f64, t25536: f64, t2775: f64, t3180: f64, t6680: f64, t7610: f64, t82596: f64, t88022: f64, t1011: f64, t4649: f64, t10474: f64, t82514: f64, t1615: f64, t3032: f64, t25483: f64, t11065: f64, t13980: f64, t13985: f64, t14590: f64, t23602: f64, t25459: f64, t25484: f64, t25485: f64, t25486: f64, t25487: f64, t25516: f64, t2780: f64, t3127: f64, t3132: f64, t4594: f64, t82513: f64, t82534: f64, t82694: f64, t362: f64, t4657: f64, t1598: f64, t1920: f64, t25535: f64, t968: f64, t1003: f64, t1058: f64, t1060: f64, t11059: f64, t14577: f64, t23658: f64, t25550: f64, t25553: f64, t25706: f64, t25723: f64, t2770: f64, t2771: f64, t7593: f64, t82668: f64, t82714: f64, t82717: f64, t88016: f64, t884: f64, t25479: f64, t82736: f64, t25493: f64, t23696: f64, t25554: f64, t25568: f64, t2776: f64, t4542: f64, t4688: f64, t6805: f64, t7611: f64, t82527: f64, t82734: f64, t82737: f64, t82739: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t89019, t89042) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2230(t25608, t6743, t1948, t6733, t23631, t61066, t974, t12652, t14586, t14595, t23323, t23327, t23609, t23657, t23673, t25502, t25510, t25511, t25512, t25523, t6797, t6799, t6800, t6801, t7603, t7615, t82539, t82555, t82643, t82657);
        let t89066 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2231(t23511, t7577, t23665, t25524, t23384, t25518, t13611, t23346, t23601, t23670, t23679, t25476, t6687, t6784, t6785, t82562, t82564, t82574, t82576, t82590, t82605);
        let t89101 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2232(t10277, t381, t225, t25608, t23384, t25714, t12648, t14165, t14644, t23327, t23346, t23613, t23686, t25429, t25456, t25470, t25510, t25511, t25517, t3010, t6687, t6786, t6797, t6799, t6800, t7614, t82618, t82629, t82633, t82635);
        let (t89106, t89143) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2233(t7604, t82573, t3961, t6746, t11046, t1409, t14213, t14571, t14630, t1629, t23327, t23511, t23613, t23633, t23635, t23657, t23678, t23685, t25429, t25540, t25544, t25717, t25722, t3120, t4347, t6687, t6784, t6797, t6799, t6800, t7619, t82661, t83239, t83240, t83245, t89019);
        let t89181 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2234(t23384, t25718, t23665, t25541, t25545, t25503, t10216, t381, t1049, t14165, t14605, t23327, t23692, t23697, t25429, t25470, t25497, t25500, t25510, t25536, t2775, t3180, t3961, t6680, t6797, t6799, t6800, t7610, t82596, t88022);
        let (t89194, t89205, t89225) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2235(t1011, t4649, t10474, t381, t82514, t1615, t3032, t25483, t23384, t25456, t1049, t11065, t13980, t13985, t14590, t23346, t23601, t23602, t25459, t25484, t25485, t25486, t25487, t25516, t25714, t2780, t3127, t3132, t4594, t6687, t6784, t7619, t82513, t82534, t82694);
        let t89265 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2236(t362, t4657, t1598, t974, t23631, t1920, t25535, t968, t1003, t1049, t1058, t1060, t11059, t14577, t23633, t23658, t25429, t25510, t25550, t25553, t25706, t25723, t2770, t2771, t2780, t3120, t3961, t6687, t6784, t6800, t7593, t7619, t82668, t82714, t82717, t83239, t88016, t884);
        let t89297 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2237(t23665, t25479, t25487, t82736, t25493, t23633, t23696, t25516, t25553, t25554, t25568, t2771, t2776, t3180, t4542, t4688, t6687, t6743, t6800, t6805, t7611, t82527, t82734, t82737, t82739);
    (t89042, t89066, t89101, t89106, t89143, t89181, t89194, t89205, t89225, t89265, t89297)
}
