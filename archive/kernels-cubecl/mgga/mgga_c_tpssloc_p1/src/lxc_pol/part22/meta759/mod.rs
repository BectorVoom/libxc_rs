//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta759 (260520-c91 hierarchical CSE).
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
mod chunk10;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2549;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2550;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2551;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2552;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2553;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2554;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2555;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2556;
use chunk8::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2557;
use chunk9::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2558;
use chunk10::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2559;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta759<F: Float>(t63332: F, t63334: F, t63336: F, t63886: F, t63888: F, t63893: F, t71124: F, t71130: F, t71135: F, t71140: F, t71142: F, t71391: F, t63911: F, t71144: F, t71400: F, t71403: F, t71406: F, t71408: F, t71411: F, t71414: F, t71417: F, t71420: F, t71423: F, t71426: F, t50846: F, t51151: F, t71146: F, t71150: F, t71152: F, t71154: F, t71156: F, t71160: F, t71166: F, t71170: F, t71174: F, t71179: F, t71183: F, t71187: F, t71446: F, t71449: F, t71452: F, t71454: F, t71456: F, t71458: F, t71461: F, t71463: F, t71465: F, t71191: F, t71195: F, t71199: F, t71468: F, t71470: F, t71472: F, t71474: F, t71477: F, t71480: F, t71483: F, t71486: F, t71489: F, t43859: F, t44027: F, t44053: F, t50919: F, t50948: F, t71203: F, t71206: F, t71499: F, t71501: F, t71505: F, t71508: F, t71511: F, t43816: F, t51039: F, t51051: F, t63361: F, t63382: F, t63384: F, t63398: F, t63400: F, t64074: F, t64076: F, t64087: F, t64089: F, t1099: F, t1118: F, t71558: F, t21813: F, t43964: F, t11310: F, t11361: F, t11365: F, t1148: F, t1155: F, t1156: F, t1683: F, t21907: F, t21939: F, t21942: F, t21947: F, t3371: F, t44205: F, t44220: F, t4857: F, t51371: F, t51677: F, t6069: F, t6088: F, t64254: F, t71530: F, t71543: F, t71545: F, t71547: F, t21938: F, t3403: F, t1117: F, t43969: F, t21810: F, t3264: F, t21809: F, t3315: F, t3313: F, t11275: F, t18265: F, t4781: F, t21723: F, t44075: F, t44077: F, t11415: F, t15126: F, t15136: F, t1682: F, t18603: F, t18606: F, t18622: F, t18643: F, t21845: F, t21906: F, t3357: F, t3376: F, t3401: F, t43692: F, t44155: F, t44223: F, t4819: F, t63502: F, t21886: F, t3359: F, t11350: F, t1136: F, t11420: F, t15146: F, t18609: F, t18612: F, t18616: F, t18619: F, t18647: F, t18650: F, t18651: F, t21854: F, t21855: F, t21887: F, t21890: F, t3332: F, t44177: F, t44179: F, t44361: F, t4840: F, t4862: F, t51604: F, t51680: F, t6056: F, t63454: F, t63602: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t71571 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2549::<F>(t63332, t63334, t63336, t63886, t63888, t63893, t71124, t71130, t71135, t71140, t71142, t71391);
        let t71585 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2550::<F>(t63911, t71144, t71400, t71403, t71406, t71408, t71411, t71414, t71417, t71420, t71423, t71426);
        let t71597 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2551::<F>(t50846, t51151, t71146, t71150, t71152, t71154, t71156, t71160, t71166, t71170, t71174, t71179);
        let (t71611, t71624) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2552::<F>(t71183, t71187, t71446, t71449, t71452, t71454, t71456, t71458, t71461, t71463, t71465, t71191, t71195, t71199, t71468, t71470, t71472, t71474, t71477, t71480, t71483, t71486, t71489);
        let t71636 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2553::<F>(t43859, t44027, t44053, t50919, t50948, t71203, t71206, t71499, t71501, t71505, t71508, t71511);
        let t71649 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2554::<F>(t43816, t51039, t51051, t63361, t63382, t63384, t63398, t63400, t64074, t64076, t64087, t64089);
        let (t71655, t71657) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2555::<F>(t1099, t1118, t71558, t71571, t71585, t71597, t71611, t71624, t71636, t71649, t21813, t43964);
        let t71664 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2556::<F>(t11310, t11361, t11365, t1148, t1155, t1156, t1683, t21907, t21939, t21942, t21947, t3371, t44205, t44220, t4857, t51371, t51677, t6069, t6088, t64254, t71530, t71543, t71545, t71547, t71655, t71657);
        let (t71672, t71697, t71700, t71704, t71707) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2557::<F>(t21938, t3403, t1117, t21813, t43969, t21810, t3264, t21809, t3315, t3313, t11275, t18265, t4781);
        let (t71711, t71712) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2558::<F>(t1117, t21723, t44075, t44077, t11310, t11415, t1155, t15126, t15136, t1682, t18603, t18606, t18622, t18643, t21845, t21906, t21939, t21942, t3357, t3376, t3401, t43692, t44155, t44223, t4819, t4857, t63502, t71672, t71697, t71700, t71704, t71707);
        let t71752 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2559::<F>(t21886, t3359, t11350, t1136, t11420, t15126, t15136, t15146, t18609, t18612, t18616, t18619, t18647, t18650, t18651, t21854, t21855, t21887, t21890, t3332, t3357, t44177, t44179, t44361, t4819, t4840, t4862, t51604, t51680, t6056, t63454, t63602);
    (t71655, t71657, t71664, t71697, t71700, t71704, t71707, t71711, t71712, t71752)
}
