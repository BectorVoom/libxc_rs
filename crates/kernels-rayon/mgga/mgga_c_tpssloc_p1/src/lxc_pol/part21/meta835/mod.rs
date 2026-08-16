//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta835 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2962;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2963;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2964;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2965;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2966;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2967;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2968;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2969;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta835(t4649: f64, t1009: f64, t17875: f64, t1011: f64, t1019: f64, t3030: f64, t5848: f64, t3032: f64, t3129: f64, t3038: f64, t10891: f64, t17655: f64, t17884: f64, t3117: f64, t18029: f64, t3108: f64, t1021: f64, t1025: f64, t10863: f64, t10957: f64, t10965: f64, t1618: f64, t17607: f64, t248: f64, t3043: f64, t3057: f64, t3064: f64, t3098: f64, t3130: f64, t3131: f64, t3134: f64, t48446: f64, t49678: f64, t5857: f64, t5861: f64, t5900: f64, t17919: f64, t3070: f64, t42488: f64, t1022: f64, t3966: f64, t360: f64, t1041: f64, t10868: f64, t5685: f64, t14134: f64, t4644: f64, t13961: f64, t4641: f64, t14137: f64, t12606: f64, t1409: f64, t10408: f64, t13555: f64, t13559: f64, t14077: f64, t1616: f64, t17632: f64, t17962: f64, t3071: f64, t3109: f64, t42743: f64, t4337: f64, t4582: f64, t4583: f64, t4652: f64, t48460: f64, t48463: f64, t5880: f64, t10413: f64, t13977: f64, t13982: f64, t13987: f64, t13991: f64, t14099: f64, t14103: f64, t14508: f64, t14511: f64, t17673: f64, t17693: f64, t3041: f64, t3048: f64, t42432: f64, t42561: f64, t4347: f64, t4650: f64, t48548: f64, t48564: f64, t48567: f64, t48570: f64, t48574: f64, t50265: f64, t5677: f64, t13969: f64, t17971: f64, t2244: f64, t5392: f64, t17713: f64, t884: f64, t1023: f64, t10390: f64, t10403: f64, t14211: f64, t17187: f64, t17688: f64, t17972: f64, t18021: f64, t3121: f64, t3132: f64, t4579: f64, t47775: f64, t48626: f64, t48629: f64, t48670: f64, t48674: f64, t50324: f64, t2250: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t61719, t61729, t61731, t61734, t61736, t61739, t61742) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2962(t4649, t1009, t17875, t1011, t1019, t3030, t5848, t3032, t3129, t3038, t10891, t17655);
        let t61760 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2963(t17884, t3117, t18029, t3108, t1021, t1025, t10863, t10957, t10965, t1618, t17607, t248, t3043, t3057, t3064, t3098, t3130, t3131, t3134, t48446, t49678, t5857, t5861, t5900, t61719, t61731, t61736, t61739, t61742);
        let (t61768, t61775, t61782, t61784) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2964(t17919, t3070, t42488, t1022, t3966, t360, t1041, t10868, t248, t5685, t14134, t4644);
        let (t61798, t61803) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2965(t13961, t4641, t14137, t4644, t12606, t1409, t10408, t1041, t10891, t13555, t13559, t14077, t1616, t17632, t17962, t3070, t3071, t3109, t42743, t4337, t4582, t4583, t4652, t48460, t48463, t5880, t61768, t61775, t61782, t61784);
        let t61835 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2966(t10408, t10413, t13977, t13982, t13987, t13991, t14099, t14103, t14508, t14511, t17673, t17693, t3041, t3048, t3070, t3071, t42432, t42561, t4347, t4650, t48548, t48564, t48567, t48570, t48574, t50265, t5677);
        let (t61853, t61855) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2967(t1041, t13969, t17971, t2244, t5392);
        let (t61871, t61876) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2968(t13969, t17713, t3130, t4649, t884, t1023, t10390, t10403, t10408, t1041, t14211, t17187, t17688, t17972, t18021, t3048, t3070, t3071, t3121, t3132, t4579, t4582, t47775, t48626, t48629, t48670, t48674, t50324, t5677, t61853, t61855);
        let t61910 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2969(t2250, t5392);
    (t61719, t61729, t61734, t61760, t61775, t61798, t61803, t61835, t61855, t61871, t61876, t61910)
}
