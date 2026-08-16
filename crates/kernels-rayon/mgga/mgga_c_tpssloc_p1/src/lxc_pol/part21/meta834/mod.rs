//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta834 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2953;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2954;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2955;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2956;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2957;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2958;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2959;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2960;
use chunk8::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2961;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta834(t12652: f64, t13536: f64, t10236: f64, t17691: f64, t13779: f64, t17183: f64, t2986: f64, t10186: f64, t10235: f64, t13769: f64, t13798: f64, t13839: f64, t13851: f64, t1539: f64, t17748: f64, t17795: f64, t23494: f64, t43055: f64, t4531: f64, t47919: f64, t47927: f64, t47941: f64, t48217: f64, t48221: f64, t48269: f64, t17863: f64, t2979: f64, t2980: f64, t43065: f64, t4514: f64, t48180: f64, t48191: f64, t48373: f64, t48378: f64, t48381: f64, t48384: f64, t48387: f64, t48390: f64, t48394: f64, t55723: f64, t59706: f64, t59711: f64, t973: f64, t16558: f64, t2989: f64, t10224: f64, t5828: f64, t42875: f64, t5817: f64, t17763: f64, t2960: f64, t10241: f64, t10245: f64, t17794: f64, t17800: f64, t2988: f64, t3014: f64, t343: f64, t4546: f64, t48397: f64, t48402: f64, t48407: f64, t48417: f64, t48421: f64, t5842: f64, t61102: f64, t61181: f64, t61214: f64, t61241: f64, t61275: f64, t61301: f64, t61332: f64, t61355: f64, t61389: f64, t61424: f64, t61453: f64, t61485: f64, t61523: f64, t18057: f64, t225: f64, t10165: f64, t1052: f64, t1065: f64, t1066: f64, t13742: f64, t14529: f64, t14545: f64, t1635: f64, t17575: f64, t18071: f64, t18074: f64, t18165: f64, t25757: f64, t3026: f64, t3169: f64, t3174: f64, t3175: f64, t3176: f64, t3207: f64, t381: f64, t388: f64, t4694: f64, t50622: f64, t50628: f64, t50690: f64, t5943: f64, t61058: f64, t61061: f64, t18059: f64, t1020: f64, t17960: f64, t248: f64, t3101: f64, t13950: f64, t4644: f64, t10508: f64, t3130: f64, t5873: f64, t17611: f64, t3114: f64, t10904: f64, t17667: f64, t1040: f64, t17877: f64, t1041: f64, t1046: f64, t10517: f64, t10863: f64, t10898: f64, t13995: f64, t14235: f64, t17890: f64, t17962: f64, t3048: f64, t3062: f64, t42522: f64, t42600: f64, t5857: f64, t5869: f64, t5875: f64, t5880: f64, t59676: f64, t3109: f64, t135: f64, t17737: f64, t10949: f64, t17607: f64, t3053: f64, t3047: f64, t5904: f64, t18030: f64, t3103: f64, t17884: f64, t10962: f64, t14085: f64, t14093: f64, t14491: f64, t1618: f64, t42570: f64, t4636: f64, t4641: f64, t48430: f64, t48441: f64, t49866: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t61524, t61560) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2953(t12652, t13536, t10236, t17691, t13779, t17183, t2986, t10186, t10235, t13769, t13798, t13839, t13851, t1539, t17748, t17795, t23494, t43055, t4531, t47919, t47927, t47941, t48217, t48221, t48269);
        let t61585 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2954(t13798, t17863, t2979, t2980, t2986, t43065, t4514, t48180, t48191, t48373, t48378, t48381, t48384, t48387, t48390, t48394, t55723, t59706, t59711, t973);
        let t61614 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2955(t16558, t2989, t10224, t5828, t973, t42875, t5817, t17763, t2960, t10241, t10245, t17794, t17800, t2986, t2988, t3014, t343, t4546, t48397, t48402, t48407, t48417, t48421, t5842);
        let t61618 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2956(t61102, t61181, t61214, t61241, t61275, t61301, t61332, t61355, t61389, t61424, t61453, t61485, t61523, t61560, t61585, t61614);
        let t61643 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2957(t18057, t225, t10165, t1052, t1065, t1066, t13742, t14529, t14545, t1635, t17575, t18071, t18074, t18165, t25757, t3026, t3169, t3174, t3175, t3176, t3207, t381, t388, t4694, t50622, t50628, t50690, t5943, t61058, t61061, t61618);
        let (t61646, t61655, t61659, t61663, t61665) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2958(t18059, t225, t1020, t17960, t248, t3101, t13950, t4644, t10508, t3130, t5873, t17611, t3114);
        let t61686 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2959(t10904, t17667, t1040, t17877, t1041, t1046, t10517, t10863, t10898, t13995, t14235, t17890, t17962, t248, t3048, t3062, t3114, t42522, t42600, t5857, t5869, t5875, t5880, t59676, t61655, t61659, t61663, t61665);
        let (t61695, t61699, t61705, t61708, t61710, t61713) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2960(t17611, t3109, t135, t17737, t973, t10949, t17667, t17607, t3053, t3047, t5904, t18030, t3103);
        let t61717 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2961(t17884, t3048, t1046, t10962, t14085, t14093, t14491, t1618, t42570, t4636, t4641, t4644, t48430, t48441, t49866, t5869, t5875, t61695, t61699, t61705, t61708, t61710, t61713);
    (t61524, t61618, t61643, t61646, t61686, t61717)
}
