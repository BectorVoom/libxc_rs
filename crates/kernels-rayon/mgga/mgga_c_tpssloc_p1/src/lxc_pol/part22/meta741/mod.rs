//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta741 (260520-c91 hierarchical CSE).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2443;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2444;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2445;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2446;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2447;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2448;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2449;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2450;
use chunk8::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2451;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta741(t13662: f64, t5791: f64, t959: f64, t21095: f64, t2940: f64, t17202: f64, t4696: f64, t4700: f64, t69036: f64, t69253: f64, t69255: f64, t69257: f64, t69259: f64, t69261: f64, t69453: f64, t69456: f64, t69263: f64, t69288: f64, t69291: f64, t69294: f64, t69297: f64, t69299: f64, t69302: f64, t69305: f64, t69307: f64, t69310: f64, t69313: f64, t21089: f64, t42110: f64, t42113: f64, t950: f64, t21370: f64, t13847: f64, t17817: f64, t2986: f64, t21444: f64, t2987: f64, t13784: f64, t21122: f64, t21456: f64, t20217: f64, t2989: f64, t20234: f64, t43070: f64, t10236: f64, t10186: f64, t13851: f64, t13861: f64, t17804: f64, t21413: f64, t21430: f64, t2988: f64, t2990: f64, t341: f64, t43069: f64, t4510: f64, t4518: f64, t4548: f64, t5836: f64, t68534: f64, t68539: f64, t68543: f64, t68547: f64, t135: f64, t21458: f64, t973: f64, t42841: f64, t4514: f64, t61189: f64, t10235: f64, t13798: f64, t17863: f64, t21433: f64, t21459: f64, t21476: f64, t2960: f64, t42811: f64, t42817: f64, t48217: f64, t61074: f64, t61172: f64, t61210: f64, t68462: f64, t68466: f64, t68470: f64, t68481: f64, t68521: f64, t21446: f64, t41863: f64, t48097: f64, t48103: f64, t68452: f64, t68454: f64, t68460: f64, t68464: f64, t68468: f64, t68472: f64, t68500: f64, t68502: f64, t68504: f64, t68506: f64, t68515: f64, t68518: f64, t68523: f64, t68527: f64, t68530: f64, t68536: f64, t68541: f64, t43002: f64, t48156: f64, t48158: f64, t60163: f64, t60168: f64, t60173: f64, t60192: f64, t60194: f64, t60202: f64, t60204: f64, t60274: f64, t60308: f64, t60310: f64, t60312: f64, t68545: f64, t68549: f64, t68552: f64, t68556: f64, t68563: f64, t68649: f64, t13536: f64, t17635: f64, t21510: f64, t13554: f64, t13769: f64, t17748: f64, t17794: f64, t17800: f64, t21447: f64, t340: f64, t343: f64, t42893: f64, t4531: f64, t48180: f64, t61094: f64, t61375: f64, t61528: f64, t61589: f64, t68477: f64, t68525: f64, t7577: f64, t884: f64, t974: f64, t13779: f64, t21126: f64, t61250: f64, t21416: f64, t21422: f64, t42903: f64, t48022: f64, t48221: f64, t5677: f64, t61086: f64, t61191: f64, t61200: f64, t61245: f64, t61252: f64, t61258: f64, t61261: f64, t61264: f64, t61273: f64, t6733: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t69459, t69461, t69462) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2443(t13662, t5791, t959, t21095, t2940, t17202, t4696, t4700, t69036, t69253, t69255, t69257, t69259, t69261, t69453, t69456);
        let (t69464, t69469) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2444(t69263, t69288, t69291, t69294, t69297, t69299, t69302, t69305, t69307, t69310, t69313, t21089, t42110, t42113, t950, t959);
        let (t69471, t69487, t69496, t69503, t69505, t69515) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2445(t21370, t2940, t13847, t17817, t2986, t21444, t2987, t13784, t21122, t21456, t20217, t2989);
        let t69533 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2446(t20234, t43070, t10236, t10186, t13851, t13861, t17804, t17817, t21413, t21430, t2986, t2988, t2990, t341, t43069, t4510, t4518, t4548, t5836, t68534, t68539, t68543, t68547, t69487, t69496, t69503, t69505, t69515);
        let t69574 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2447(t135, t21458, t973, t20234, t42841, t2986, t4514, t61189, t10186, t10235, t13798, t17863, t21433, t21459, t21476, t2960, t42811, t42817, t4510, t48217, t61074, t61172, t61210, t68462, t68466, t68470, t68481, t68521);
        let (t69579, t69598) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2448(t135, t21446, t973, t41863, t48097, t48103, t68452, t68454, t68460, t68464, t68468, t68472, t68500, t68502, t68504, t68506, t68515, t68518, t68523, t68527, t68530, t68536, t68541);
        let t69615 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2449(t43002, t48156, t48158, t60163, t60168, t60173, t60192, t60194, t60202, t60204, t60274, t60308, t60310, t60312, t68545, t68549, t68552, t68556, t68563, t68649);
        let (t69643, t69657, t69665) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2450(t13536, t17635, t10236, t21510, t13554, t10235, t13769, t13798, t13851, t13861, t17748, t17794, t17800, t21447, t2960, t2986, t340, t343, t42893, t4510, t4531, t48180, t61094, t61375, t61528, t61589, t68477, t68525, t69579, t69598, t69615, t7577, t884, t973, t974);
        let t69695 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2451(t13779, t21126, t2986, t4514, t61250, t13847, t17794, t10186, t13769, t21416, t21422, t42903, t48022, t48221, t5677, t61086, t61191, t61200, t61245, t61252, t61258, t61261, t61264, t61273, t6733);
    (t69459, t69461, t69462, t69464, t69469, t69471, t69533, t69574, t69643, t69657, t69665, t69695)
}
