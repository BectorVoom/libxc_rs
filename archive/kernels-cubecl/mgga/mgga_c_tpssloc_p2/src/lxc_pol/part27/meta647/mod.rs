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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2230;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2231;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2232;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2233;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2234;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2235;
use chunk6::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2236;
use chunk7::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2237;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta647<F: Float>(t25608: F, t6743: F, t1948: F, t6733: F, t23631: F, t61066: F, t974: F, t12652: F, t14586: F, t14595: F, t23323: F, t23327: F, t23609: F, t23657: F, t23673: F, t25502: F, t25510: F, t25511: F, t25512: F, t25523: F, t6797: F, t6799: F, t6800: F, t6801: F, t7603: F, t7615: F, t82539: F, t82555: F, t82643: F, t82657: F, t23511: F, t7577: F, t23665: F, t25524: F, t23384: F, t25518: F, t13611: F, t23346: F, t23601: F, t23670: F, t23679: F, t25476: F, t6687: F, t6784: F, t6785: F, t82562: F, t82564: F, t82574: F, t82576: F, t82590: F, t82605: F, t10277: F, t381: F, t225: F, t25714: F, t12648: F, t14165: F, t14644: F, t23613: F, t23686: F, t25429: F, t25456: F, t25470: F, t25517: F, t3010: F, t6786: F, t7614: F, t82618: F, t82629: F, t82633: F, t82635: F, t7604: F, t82573: F, t3961: F, t6746: F, t11046: F, t1409: F, t14213: F, t14571: F, t14630: F, t1629: F, t23633: F, t23635: F, t23678: F, t23685: F, t25540: F, t25544: F, t25717: F, t25722: F, t3120: F, t4347: F, t7619: F, t82661: F, t83239: F, t83240: F, t83245: F, t25718: F, t25541: F, t25545: F, t25503: F, t10216: F, t1049: F, t14605: F, t23692: F, t23697: F, t25497: F, t25500: F, t25536: F, t2775: F, t3180: F, t6680: F, t7610: F, t82596: F, t88022: F, t1011: F, t4649: F, t10474: F, t82514: F, t1615: F, t3032: F, t25483: F, t11065: F, t13980: F, t13985: F, t14590: F, t23602: F, t25459: F, t25484: F, t25485: F, t25486: F, t25487: F, t25516: F, t2780: F, t3127: F, t3132: F, t4594: F, t82513: F, t82534: F, t82694: F, t362: F, t4657: F, t1598: F, t1920: F, t25535: F, t968: F, t1003: F, t1058: F, t1060: F, t11059: F, t14577: F, t23658: F, t25550: F, t25553: F, t25706: F, t25723: F, t2770: F, t2771: F, t7593: F, t82668: F, t82714: F, t82717: F, t88016: F, t884: F, t25479: F, t82736: F, t25493: F, t23696: F, t25554: F, t25568: F, t2776: F, t4542: F, t4688: F, t6805: F, t7611: F, t82527: F, t82734: F, t82737: F, t82739: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t89019, t89042) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2230::<F>(t25608, t6743, t1948, t6733, t23631, t61066, t974, t12652, t14586, t14595, t23323, t23327, t23609, t23657, t23673, t25502, t25510, t25511, t25512, t25523, t6797, t6799, t6800, t6801, t7603, t7615, t82539, t82555, t82643, t82657);
        let t89066 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2231::<F>(t23511, t7577, t23665, t25524, t23384, t25518, t13611, t23346, t23601, t23670, t23679, t25476, t6687, t6784, t6785, t82562, t82564, t82574, t82576, t82590, t82605);
        let t89101 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2232::<F>(t10277, t381, t225, t25608, t23384, t25714, t12648, t14165, t14644, t23327, t23346, t23613, t23686, t25429, t25456, t25470, t25510, t25511, t25517, t3010, t6687, t6786, t6797, t6799, t6800, t7614, t82618, t82629, t82633, t82635);
        let (t89106, t89143) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2233::<F>(t7604, t82573, t3961, t6746, t11046, t1409, t14213, t14571, t14630, t1629, t23327, t23511, t23613, t23633, t23635, t23657, t23678, t23685, t25429, t25540, t25544, t25717, t25722, t3120, t4347, t6687, t6784, t6797, t6799, t6800, t7619, t82661, t83239, t83240, t83245, t89019);
        let t89181 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2234::<F>(t23384, t25718, t23665, t25541, t25545, t25503, t10216, t381, t1049, t14165, t14605, t23327, t23692, t23697, t25429, t25470, t25497, t25500, t25510, t25536, t2775, t3180, t3961, t6680, t6797, t6799, t6800, t7610, t82596, t88022);
        let (t89194, t89205, t89225) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2235::<F>(t1011, t4649, t10474, t381, t82514, t1615, t3032, t25483, t23384, t25456, t1049, t11065, t13980, t13985, t14590, t23346, t23601, t23602, t25459, t25484, t25485, t25486, t25487, t25516, t25714, t2780, t3127, t3132, t4594, t6687, t6784, t7619, t82513, t82534, t82694);
        let t89265 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2236::<F>(t362, t4657, t1598, t974, t23631, t1920, t25535, t968, t1003, t1049, t1058, t1060, t11059, t14577, t23633, t23658, t25429, t25510, t25550, t25553, t25706, t25723, t2770, t2771, t2780, t3120, t3961, t6687, t6784, t6800, t7593, t7619, t82668, t82714, t82717, t83239, t88016, t884);
        let t89297 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2237::<F>(t23665, t25479, t25487, t82736, t25493, t23633, t23696, t25516, t25553, t25554, t25568, t2771, t2776, t3180, t4542, t4688, t6687, t6743, t6800, t6805, t7611, t82527, t82734, t82737, t82739);
    (t89042, t89066, t89101, t89106, t89143, t89181, t89194, t89205, t89225, t89265, t89297)
}
