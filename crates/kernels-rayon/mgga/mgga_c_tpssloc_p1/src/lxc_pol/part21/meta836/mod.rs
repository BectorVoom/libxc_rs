//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta836 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2970;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2971;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2972;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2973;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2974;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2975;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2976;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2977;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta836(t17997: f64, t3070: f64, t42488: f64, t1041: f64, t13969: f64, t17975: f64, t10413: f64, t10876: f64, t10937: f64, t14080: f64, t1409: f64, t14167: f64, t14172: f64, t14218: f64, t14219: f64, t17649: f64, t17712: f64, t17920: f64, t17923: f64, t3071: f64, t3131: f64, t3132: f64, t3966: f64, t42483: f64, t43361: f64, t4579: f64, t4582: f64, t4590: f64, t4644: f64, t49604: f64, t49607: f64, t49621: f64, t49629: f64, t49984: f64, t61910: f64, t883: f64, t17687: f64, t14085: f64, t4571: f64, t13765: f64, t13995: f64, t18086: f64, t3069: f64, t10403: f64, t10891: f64, t14041: f64, t14130: f64, t17718: f64, t2776: f64, t3041: f64, t3073: f64, t3121: f64, t4650: f64, t47779: f64, t48611: f64, t49658: f64, t49661: f64, t49666: f64, t5685: f64, t5867: f64, t61855: f64, t10952: f64, t17655: f64, t17659: f64, t3117: f64, t17187: f64, t248: f64, t3051: f64, t10390: f64, t10480: f64, t10904: f64, t13762: f64, t14488: f64, t17670: f64, t17714: f64, t17998: f64, t3040: f64, t3130: f64, t42552: f64, t42573: f64, t43291: f64, t43292: f64, t4593: f64, t4596: f64, t48607: f64, t49651: f64, t49682: f64, t49684: f64, t50510: f64, t5880: f64, t61078: f64, t10422: f64, t17704: f64, t17680: f64, t17692: f64, t10408: f64, t17697: f64, t17705: f64, t17984: f64, t2771: f64, t3048: f64, t42334: f64, t42388: f64, t42586: f64, t4575: f64, t4600: f64, t48477: f64, t48612: f64, t49690: f64, t49692: f64, t49697: f64, t49771: f64, t5878: f64, t61098: f64, t2244: f64, t5398: f64, t14077: f64, t4630: f64, t10401: f64, t246: f64, t3067: f64, t3186: f64, t1615: f64, t3061: f64, t375: f64, t1022: f64, t3961: f64, t3200: f64, t10482: f64, t5872: f64, t17924: f64, t17959: f64, t376: f64, t17672: f64, t14164: f64, t14207: f64, t14213: f64, t14228: f64, t14234: f64, t17151: f64, t17177: f64, t17182: f64, t17673: f64, t17925: f64, t2770: f64, t42397: f64, t42508: f64, t43322: f64, t4594: f64, t4652: f64, t49702: f64, t10868: f64, t5681: f64, t10949: f64, t14187: f64, t1622: f64, t17643: f64, t17734: f64, t17972: f64, t4583: f64, t4588: f64, t4636: f64, t49716: f64, t49721: f64, t49732: f64, t49740: f64, t50334: f64, t55662: f64, t55666: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t61921 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2970(t17997, t3070, t42488, t1041, t13969, t17975, t10413, t10876, t10937, t14080, t1409, t14167, t14172, t14218, t14219, t17649, t17712, t17920, t17923, t3071, t3131, t3132, t3966, t42483, t43361, t4579, t4582, t4590, t4644, t49604, t49607, t49621, t49629, t49984, t61910, t883);
        let t61965 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2971(t1041, t13969, t17687, t14085, t4571, t13765, t13995, t18086, t3069, t10403, t10413, t10891, t14041, t14130, t14218, t17718, t2776, t3041, t3070, t3071, t3073, t3121, t3132, t42483, t4582, t4650, t47779, t48611, t49658, t49661, t49666, t5685, t5867, t61855);
        let t62007 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2972(t10952, t17655, t17659, t3117, t1041, t17187, t248, t3051, t10390, t10480, t10904, t13762, t13995, t14488, t17670, t17714, t17998, t3040, t3071, t3130, t3131, t42552, t42573, t43291, t43292, t4582, t4593, t4596, t48607, t49651, t49682, t49684, t50510, t5880, t61078);
        let t62042 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2973(t10422, t17704, t3070, t17680, t1041, t13969, t17692, t10408, t10413, t10937, t17697, t17705, t17984, t2771, t3048, t42334, t42388, t42586, t4575, t4600, t48477, t48607, t48611, t48612, t49690, t49692, t49697, t49771, t49984, t5878, t61098);
        let t62044 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2974(t2244, t5398);
        let (t62049, t62053, t62055, t62057, t62059, t62064) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2975(t14077, t4630, t10401, t246, t3067, t3186, t1615, t3061, t375, t1022, t3961, t3200);
        let (t62091, t62101) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2976(t10482, t5872, t10413, t10422, t17924, t17959, t376, t10480, t13969, t17672, t10408, t1041, t14164, t14207, t14213, t14228, t14234, t17151, t17177, t17182, t17673, t17925, t2770, t3070, t3071, t3130, t3131, t42388, t42397, t42508, t43322, t4582, t4594, t4652, t49702, t62044, t62049, t62055, t62057, t62059, t62064);
        let t62145 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2977(t1041, t10868, t248, t5681, t10949, t14080, t14172, t14187, t1622, t17643, t17734, t17972, t3117, t4582, t4583, t4588, t4636, t49716, t49721, t49732, t49740, t50334, t55662, t55666, t62044);
    (t61921, t61965, t62007, t62042, t62044, t62053, t62055, t62059, t62064, t62091, t62101, t62145)
}
