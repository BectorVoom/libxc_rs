//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta826 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2912;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2913;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2914;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2915;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2916;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2917;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta826(t59748: f64, t59753: f64, t59757: f64, t59759: f64, t59761: f64, t59765: f64, t59769: f64, t60308: f64, t60310: f64, t60312: f64, t60315: f64, t60318: f64, t60321: f64, t60324: f64, t60327: f64, t10760: f64, t10828: f64, t14266: f64, t14329: f64, t1569: f64, t17350: f64, t17428: f64, t17499: f64, t2856: f64, t2881: f64, t2889: f64, t2906: f64, t2907: f64, t2924: f64, t2930: f64, t2932: f64, t41816: f64, t41826: f64, t41981: f64, t4411: f64, t4434: f64, t48771: f64, t48779: f64, t48890: f64, t5743: f64, t5759: f64, t5794: f64, t59975: f64, t60407: f64, t60424: f64, t60429: f64, t60434: f64, t60568: f64, t60570: f64, t60585: f64, t60601: f64, t60618: f64, t60634: f64, t60649: f64, t60665: f64, t60682: f64, t924: f64, t932: f64, t950: f64, t10632: f64, t5790: f64, t10655: f64, t17521: f64, t17423: f64, t2792: f64, t912: f64, t17422: f64, t2844: f64, t2842: f64, t17524: f64, t17528: f64, t42023: f64, t10756: f64, t10765: f64, t13716: f64, t14271: f64, t14276: f64, t14425: f64, t14429: f64, t14432: f64, t14436: f64, t17492: f64, t17535: f64, t2905: f64, t42111: f64, t42113: f64, t4416: f64, t4438: f64, t4475: f64, t48789: f64, t49427: f64, t49430: f64, t5774: f64, t5791: f64, t17349: f64, t2888: f64, t13520: f64, t14422: f64, t10740: f64, t10747: f64, t10825: f64, t14263: f64, t14337: f64, t14450: f64, t14460: f64, t17443: f64, t17446: f64, t17451: f64, t17454: f64, t17493: f64, t17538: f64, t17541: f64, t17544: f64, t17548: f64, t17551: f64, t17555: f64, t2861: f64, t2886: f64, t41984: f64, t42128: f64, t42149: f64, t4454: f64, t4476: f64, t49096: f64, t49411: f64, t60360: f64, t931: f64, t300: f64, t59928: f64, t59982: f64, t60030: f64, t60346: f64, t60401: f64, t17955: f64, t2940: f64, t17930: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t60698 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2912(t59748, t59753, t59757, t59759, t59761, t59765, t59769, t60308, t60310, t60312, t60315, t60318, t60321, t60324, t60327);
        let t60711 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2913(t10760, t10828, t14266, t14329, t1569, t17350, t17428, t17499, t2856, t2881, t2889, t2906, t2907, t2924, t2930, t2932, t41816, t41826, t41981, t4411, t4434, t48771, t48779, t48890, t5743, t5759, t5794, t59975, t60407, t60424, t60429, t60434, t60568, t60570, t60585, t60601, t60618, t60634, t60649, t60665, t60682, t60698, t924, t932, t950);
        let (t60722, t60741, t60744, t60748, t60750, t60752) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2914(t10632, t5790, t10655, t17521, t17423, t2792, t912, t17422, t2844, t2842, t17524, t17528, t42023);
        let t60763 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2915(t10756, t10765, t10828, t13716, t14271, t14276, t14425, t14429, t14432, t14436, t17492, t17499, t17535, t2905, t2906, t2924, t2930, t42111, t42113, t4416, t4438, t4475, t48789, t49427, t49430, t5774, t5791, t60722, t60741, t60744, t60748, t60750, t60752);
        let (t60787, t60806) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2916(t17349, t2888, t13520, t14422, t10740, t10747, t10765, t10825, t14263, t14337, t14450, t14460, t17350, t17443, t17446, t17451, t17454, t17493, t17538, t17541, t17544, t17548, t17551, t17555, t2861, t2886, t41984, t42128, t42149, t4454, t4476, t49096, t49411, t60360, t931, t932);
        let (t60810, t60812, t60814) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2917(t300, t59928, t59982, t60030, t60346, t60401, t60711, t60763, t60806, t17955, t2940, t17930);
    (t60741, t60744, t60748, t60750, t60752, t60787, t60810, t60812, t60814)
}
