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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta834<F: Float>(t12652: F, t13536: F, t10236: F, t17691: F, t13779: F, t17183: F, t2986: F, t10186: F, t10235: F, t13769: F, t13798: F, t13839: F, t13851: F, t1539: F, t17748: F, t17795: F, t23494: F, t43055: F, t4531: F, t47919: F, t47927: F, t47941: F, t48217: F, t48221: F, t48269: F, t17863: F, t2979: F, t2980: F, t43065: F, t4514: F, t48180: F, t48191: F, t48373: F, t48378: F, t48381: F, t48384: F, t48387: F, t48390: F, t48394: F, t55723: F, t59706: F, t59711: F, t973: F, t16558: F, t2989: F, t10224: F, t5828: F, t42875: F, t5817: F, t17763: F, t2960: F, t10241: F, t10245: F, t17794: F, t17800: F, t2988: F, t3014: F, t343: F, t4546: F, t48397: F, t48402: F, t48407: F, t48417: F, t48421: F, t5842: F, t61102: F, t61181: F, t61214: F, t61241: F, t61275: F, t61301: F, t61332: F, t61355: F, t61389: F, t61424: F, t61453: F, t61485: F, t61523: F, t18057: F, t225: F, t10165: F, t1052: F, t1065: F, t1066: F, t13742: F, t14529: F, t14545: F, t1635: F, t17575: F, t18071: F, t18074: F, t18165: F, t25757: F, t3026: F, t3169: F, t3174: F, t3175: F, t3176: F, t3207: F, t381: F, t388: F, t4694: F, t50622: F, t50628: F, t50690: F, t5943: F, t61058: F, t61061: F, t18059: F, t1020: F, t17960: F, t248: F, t3101: F, t13950: F, t4644: F, t10508: F, t3130: F, t5873: F, t17611: F, t3114: F, t10904: F, t17667: F, t1040: F, t17877: F, t1041: F, t1046: F, t10517: F, t10863: F, t10898: F, t13995: F, t14235: F, t17890: F, t17962: F, t3048: F, t3062: F, t42522: F, t42600: F, t5857: F, t5869: F, t5875: F, t5880: F, t59676: F, t3109: F, t135: F, t17737: F, t10949: F, t17607: F, t3053: F, t3047: F, t5904: F, t18030: F, t3103: F, t17884: F, t10962: F, t14085: F, t14093: F, t14491: F, t1618: F, t42570: F, t4636: F, t4641: F, t48430: F, t48441: F, t49866: F) -> (F, F, F, F, F, F) {
        let (t61524, t61560) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2953::<F>(t12652, t13536, t10236, t17691, t13779, t17183, t2986, t10186, t10235, t13769, t13798, t13839, t13851, t1539, t17748, t17795, t23494, t43055, t4531, t47919, t47927, t47941, t48217, t48221, t48269);
        let t61585 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2954::<F>(t13798, t17863, t2979, t2980, t2986, t43065, t4514, t48180, t48191, t48373, t48378, t48381, t48384, t48387, t48390, t48394, t55723, t59706, t59711, t973);
        let t61614 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2955::<F>(t16558, t2989, t10224, t5828, t973, t42875, t5817, t17763, t2960, t10241, t10245, t17794, t17800, t2986, t2988, t3014, t343, t4546, t48397, t48402, t48407, t48417, t48421, t5842);
        let t61618 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2956::<F>(t61102, t61181, t61214, t61241, t61275, t61301, t61332, t61355, t61389, t61424, t61453, t61485, t61523, t61560, t61585, t61614);
        let t61643 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2957::<F>(t18057, t225, t10165, t1052, t1065, t1066, t13742, t14529, t14545, t1635, t17575, t18071, t18074, t18165, t25757, t3026, t3169, t3174, t3175, t3176, t3207, t381, t388, t4694, t50622, t50628, t50690, t5943, t61058, t61061, t61618);
        let (t61646, t61655, t61659, t61663, t61665) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2958::<F>(t18059, t225, t1020, t17960, t248, t3101, t13950, t4644, t10508, t3130, t5873, t17611, t3114);
        let t61686 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2959::<F>(t10904, t17667, t1040, t17877, t1041, t1046, t10517, t10863, t10898, t13995, t14235, t17890, t17962, t248, t3048, t3062, t3114, t42522, t42600, t5857, t5869, t5875, t5880, t59676, t61655, t61659, t61663, t61665);
        let (t61695, t61699, t61705, t61708, t61710, t61713) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2960::<F>(t17611, t3109, t135, t17737, t973, t10949, t17667, t17607, t3053, t3047, t5904, t18030, t3103);
        let t61717 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2961::<F>(t17884, t3048, t1046, t10962, t14085, t14093, t14491, t1618, t42570, t4636, t4641, t4644, t48430, t48441, t49866, t5869, t5875, t61695, t61699, t61705, t61708, t61710, t61713);
    (t61524, t61618, t61643, t61646, t61686, t61717)
}
