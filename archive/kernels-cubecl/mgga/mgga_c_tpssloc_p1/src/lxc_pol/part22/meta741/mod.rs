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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta741<F: Float>(t13662: F, t5791: F, t959: F, t21095: F, t2940: F, t17202: F, t4696: F, t4700: F, t69036: F, t69253: F, t69255: F, t69257: F, t69259: F, t69261: F, t69453: F, t69456: F, t69263: F, t69288: F, t69291: F, t69294: F, t69297: F, t69299: F, t69302: F, t69305: F, t69307: F, t69310: F, t69313: F, t21089: F, t42110: F, t42113: F, t950: F, t21370: F, t13847: F, t17817: F, t2986: F, t21444: F, t2987: F, t13784: F, t21122: F, t21456: F, t20217: F, t2989: F, t20234: F, t43070: F, t10236: F, t10186: F, t13851: F, t13861: F, t17804: F, t21413: F, t21430: F, t2988: F, t2990: F, t341: F, t43069: F, t4510: F, t4518: F, t4548: F, t5836: F, t68534: F, t68539: F, t68543: F, t68547: F, t135: F, t21458: F, t973: F, t42841: F, t4514: F, t61189: F, t10235: F, t13798: F, t17863: F, t21433: F, t21459: F, t21476: F, t2960: F, t42811: F, t42817: F, t48217: F, t61074: F, t61172: F, t61210: F, t68462: F, t68466: F, t68470: F, t68481: F, t68521: F, t21446: F, t41863: F, t48097: F, t48103: F, t68452: F, t68454: F, t68460: F, t68464: F, t68468: F, t68472: F, t68500: F, t68502: F, t68504: F, t68506: F, t68515: F, t68518: F, t68523: F, t68527: F, t68530: F, t68536: F, t68541: F, t43002: F, t48156: F, t48158: F, t60163: F, t60168: F, t60173: F, t60192: F, t60194: F, t60202: F, t60204: F, t60274: F, t60308: F, t60310: F, t60312: F, t68545: F, t68549: F, t68552: F, t68556: F, t68563: F, t68649: F, t13536: F, t17635: F, t21510: F, t13554: F, t13769: F, t17748: F, t17794: F, t17800: F, t21447: F, t340: F, t343: F, t42893: F, t4531: F, t48180: F, t61094: F, t61375: F, t61528: F, t61589: F, t68477: F, t68525: F, t7577: F, t884: F, t974: F, t13779: F, t21126: F, t61250: F, t21416: F, t21422: F, t42903: F, t48022: F, t48221: F, t5677: F, t61086: F, t61191: F, t61200: F, t61245: F, t61252: F, t61258: F, t61261: F, t61264: F, t61273: F, t6733: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t69459, t69461, t69462) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2443::<F>(t13662, t5791, t959, t21095, t2940, t17202, t4696, t4700, t69036, t69253, t69255, t69257, t69259, t69261, t69453, t69456);
        let (t69464, t69469) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2444::<F>(t69263, t69288, t69291, t69294, t69297, t69299, t69302, t69305, t69307, t69310, t69313, t21089, t42110, t42113, t950, t959);
        let (t69471, t69487, t69496, t69503, t69505, t69515) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2445::<F>(t21370, t2940, t13847, t17817, t2986, t21444, t2987, t13784, t21122, t21456, t20217, t2989);
        let t69533 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2446::<F>(t20234, t43070, t10236, t10186, t13851, t13861, t17804, t17817, t21413, t21430, t2986, t2988, t2990, t341, t43069, t4510, t4518, t4548, t5836, t68534, t68539, t68543, t68547, t69487, t69496, t69503, t69505, t69515);
        let t69574 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2447::<F>(t135, t21458, t973, t20234, t42841, t2986, t4514, t61189, t10186, t10235, t13798, t17863, t21433, t21459, t21476, t2960, t42811, t42817, t4510, t48217, t61074, t61172, t61210, t68462, t68466, t68470, t68481, t68521);
        let (t69579, t69598) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2448::<F>(t135, t21446, t973, t41863, t48097, t48103, t68452, t68454, t68460, t68464, t68468, t68472, t68500, t68502, t68504, t68506, t68515, t68518, t68523, t68527, t68530, t68536, t68541);
        let t69615 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2449::<F>(t43002, t48156, t48158, t60163, t60168, t60173, t60192, t60194, t60202, t60204, t60274, t60308, t60310, t60312, t68545, t68549, t68552, t68556, t68563, t68649);
        let (t69643, t69657, t69665) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2450::<F>(t13536, t17635, t10236, t21510, t13554, t10235, t13769, t13798, t13851, t13861, t17748, t17794, t17800, t21447, t2960, t2986, t340, t343, t42893, t4510, t4531, t48180, t61094, t61375, t61528, t61589, t68477, t68525, t69579, t69598, t69615, t7577, t884, t973, t974);
        let t69695 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2451::<F>(t13779, t21126, t2986, t4514, t61250, t13847, t17794, t10186, t13769, t21416, t21422, t42903, t48022, t48221, t5677, t61086, t61191, t61200, t61245, t61252, t61258, t61261, t61264, t61273, t6733);
    (t69459, t69461, t69462, t69464, t69469, t69471, t69533, t69574, t69643, t69657, t69665, t69695)
}
