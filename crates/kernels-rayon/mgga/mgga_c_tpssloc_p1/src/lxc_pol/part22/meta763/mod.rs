//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta763 (260520-c91 hierarchical CSE).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2570;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2571;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2572;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2573;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2574;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2575;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2576;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2577;
use chunk8::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2578;
use chunk9::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2579;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta763(t71183: f64, t71187: f64, t71446: f64, t71449: f64, t71452: f64, t71454: f64, t71456: f64, t71458: f64, t71461: f64, t71463: f64, t71465: f64, t71191: f64, t71195: f64, t71199: f64, t71468: f64, t71470: f64, t71472: f64, t71474: f64, t71477: f64, t71480: f64, t71483: f64, t71486: f64, t71489: f64, t43859: f64, t44249: f64, t44275: f64, t51299: f64, t51310: f64, t71203: f64, t71206: f64, t71499: f64, t71501: f64, t71505: f64, t71508: f64, t71511: f64, t43816: f64, t51349: f64, t51354: f64, t63361: f64, t63382: f64, t63384: f64, t63398: f64, t63400: f64, t64074: f64, t64076: f64, t64087: f64, t64089: f64, t50834: f64, t51745: f64, t63291: f64, t63306: f64, t63308: f64, t63332: f64, t63334: f64, t63336: f64, t71124: f64, t71130: f64, t71135: f64, t71140: f64, t71142: f64, t71144: f64, t71146: f64, t71150: f64, t71152: f64, t71154: f64, t71156: f64, t71160: f64, t44320: f64, t51760: f64, t51769: f64, t71166: f64, t71170: f64, t71174: f64, t71179: f64, t1129: f64, t1137: f64, t15121: f64, t15141: f64, t1695: f64, t18644: f64, t18840: f64, t18894: f64, t18899: f64, t21855: f64, t21887: f64, t21890: f64, t3327: f64, t436: f64, t44172: f64, t44214: f64, t4797: f64, t4820: f64, t4858: f64, t51392: f64, t51599: f64, t6053: f64, t6056: f64, t6085: f64, t63597: f64, t71876: f64, t71879: f64, t71902: f64, t71915: f64, t71929: f64, t71941: f64, t300: f64, t71322: f64, t71664: f64, t71712: f64, t71752: f64, t71791: f64, t71828: f64, t71868: f64, t18926: f64, t4869: f64, t1164: f64, t14960: f64, t14858: f64, t6102: f64, t1157: f64, t22228: f64, t1763: f64, t4700: f64, t64548: f64, t71255: f64, t71313: f64, t71315: f64, t71317: f64, t71319: f64, t18915: f64, t4879: f64, t21938: f64, t3400: f64, t4883: f64, t71310: f64, t1155: f64, t51810: f64, t6084: f64, t1703: f64, t65288: f64, t71543: f64, t71545: f64, t71547: f64, t71655: f64, t71657: f64, t71697: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t71955, t71968) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2570(t71183, t71187, t71446, t71449, t71452, t71454, t71456, t71458, t71461, t71463, t71465, t71191, t71195, t71199, t71468, t71470, t71472, t71474, t71477, t71480, t71483, t71486, t71489);
        let t71978 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2571(t43859, t44249, t44275, t51299, t51310, t71203, t71206, t71499, t71501, t71505, t71508, t71511);
        let t71989 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2572(t43816, t51349, t51354, t63361, t63382, t63384, t63398, t63400, t64074, t64076, t64087, t64089);
        let t72019 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2573(t50834, t51745, t63291, t63306, t63308, t63332, t63334, t63336, t71124, t71130, t71135, t71140, t71142, t71144, t71146, t71150, t71152, t71154, t71156, t71160);
        let t72037 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2574(t43816, t44320, t51760, t51769, t63361, t63382, t63384, t63398, t63400, t71166, t71170, t71174, t71179, t71183, t71187, t71191, t71195, t71199, t71203, t71206);
        let t72041 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2575(t1129, t1137, t15121, t15141, t1695, t18644, t18840, t18894, t18899, t21855, t21887, t21890, t3327, t436, t44172, t44214, t4797, t4820, t4858, t51392, t51599, t6053, t6056, t6085, t63597, t71876, t71879, t71902, t71915, t71929, t71941, t71955, t71968, t71978, t71989, t72019, t72037);
        let (t72045, t72047, t72050) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2576(t300, t71322, t71664, t71712, t71752, t71791, t71828, t71868, t72041, t18926, t4869, t1164, t14960, t6085);
        let (t72052, t72058, t72059) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2577(t14858, t6102, t1157, t1164, t22228, t1763, t4700, t64548, t71255, t71313, t71315, t71317, t71319, t72045, t72047, t72050);
        let (t72061, t72065, t72067, t72071) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2578(t18915, t4879, t21938, t3400, t1164, t4883, t300, t71310, t1155, t1695, t51810, t6084);
        let (t72073, t72074) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2579(t1703, t65288, t71543, t71545, t71547, t71655, t71657, t71697, t72061, t72065, t72067, t72071);
    (t72045, t72047, t72050, t72052, t72058, t72059, t72061, t72065, t72067, t72071, t72073, t72074)
}
