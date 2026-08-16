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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta763<F: Float>(t71183: F, t71187: F, t71446: F, t71449: F, t71452: F, t71454: F, t71456: F, t71458: F, t71461: F, t71463: F, t71465: F, t71191: F, t71195: F, t71199: F, t71468: F, t71470: F, t71472: F, t71474: F, t71477: F, t71480: F, t71483: F, t71486: F, t71489: F, t43859: F, t44249: F, t44275: F, t51299: F, t51310: F, t71203: F, t71206: F, t71499: F, t71501: F, t71505: F, t71508: F, t71511: F, t43816: F, t51349: F, t51354: F, t63361: F, t63382: F, t63384: F, t63398: F, t63400: F, t64074: F, t64076: F, t64087: F, t64089: F, t50834: F, t51745: F, t63291: F, t63306: F, t63308: F, t63332: F, t63334: F, t63336: F, t71124: F, t71130: F, t71135: F, t71140: F, t71142: F, t71144: F, t71146: F, t71150: F, t71152: F, t71154: F, t71156: F, t71160: F, t44320: F, t51760: F, t51769: F, t71166: F, t71170: F, t71174: F, t71179: F, t1129: F, t1137: F, t15121: F, t15141: F, t1695: F, t18644: F, t18840: F, t18894: F, t18899: F, t21855: F, t21887: F, t21890: F, t3327: F, t436: F, t44172: F, t44214: F, t4797: F, t4820: F, t4858: F, t51392: F, t51599: F, t6053: F, t6056: F, t6085: F, t63597: F, t71876: F, t71879: F, t71902: F, t71915: F, t71929: F, t71941: F, t300: F, t71322: F, t71664: F, t71712: F, t71752: F, t71791: F, t71828: F, t71868: F, t18926: F, t4869: F, t1164: F, t14960: F, t14858: F, t6102: F, t1157: F, t22228: F, t1763: F, t4700: F, t64548: F, t71255: F, t71313: F, t71315: F, t71317: F, t71319: F, t18915: F, t4879: F, t21938: F, t3400: F, t4883: F, t71310: F, t1155: F, t51810: F, t6084: F, t1703: F, t65288: F, t71543: F, t71545: F, t71547: F, t71655: F, t71657: F, t71697: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t71955, t71968) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2570::<F>(t71183, t71187, t71446, t71449, t71452, t71454, t71456, t71458, t71461, t71463, t71465, t71191, t71195, t71199, t71468, t71470, t71472, t71474, t71477, t71480, t71483, t71486, t71489);
        let t71978 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2571::<F>(t43859, t44249, t44275, t51299, t51310, t71203, t71206, t71499, t71501, t71505, t71508, t71511);
        let t71989 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2572::<F>(t43816, t51349, t51354, t63361, t63382, t63384, t63398, t63400, t64074, t64076, t64087, t64089);
        let t72019 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2573::<F>(t50834, t51745, t63291, t63306, t63308, t63332, t63334, t63336, t71124, t71130, t71135, t71140, t71142, t71144, t71146, t71150, t71152, t71154, t71156, t71160);
        let t72037 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2574::<F>(t43816, t44320, t51760, t51769, t63361, t63382, t63384, t63398, t63400, t71166, t71170, t71174, t71179, t71183, t71187, t71191, t71195, t71199, t71203, t71206);
        let t72041 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2575::<F>(t1129, t1137, t15121, t15141, t1695, t18644, t18840, t18894, t18899, t21855, t21887, t21890, t3327, t436, t44172, t44214, t4797, t4820, t4858, t51392, t51599, t6053, t6056, t6085, t63597, t71876, t71879, t71902, t71915, t71929, t71941, t71955, t71968, t71978, t71989, t72019, t72037);
        let (t72045, t72047, t72050) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2576::<F>(t300, t71322, t71664, t71712, t71752, t71791, t71828, t71868, t72041, t18926, t4869, t1164, t14960, t6085);
        let (t72052, t72058, t72059) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2577::<F>(t14858, t6102, t1157, t1164, t22228, t1763, t4700, t64548, t71255, t71313, t71315, t71317, t71319, t72045, t72047, t72050);
        let (t72061, t72065, t72067, t72071) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2578::<F>(t18915, t4879, t21938, t3400, t1164, t4883, t300, t71310, t1155, t1695, t51810, t6084);
        let (t72073, t72074) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2579::<F>(t1703, t65288, t71543, t71545, t71547, t71655, t71657, t71697, t72061, t72065, t72067, t72071);
    (t72045, t72047, t72050, t72052, t72058, t72059, t72061, t72065, t72067, t72071, t72073, t72074)
}
