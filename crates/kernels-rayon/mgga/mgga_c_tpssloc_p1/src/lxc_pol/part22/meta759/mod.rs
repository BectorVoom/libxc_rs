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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta759(t63332: f64, t63334: f64, t63336: f64, t63886: f64, t63888: f64, t63893: f64, t71124: f64, t71130: f64, t71135: f64, t71140: f64, t71142: f64, t71391: f64, t63911: f64, t71144: f64, t71400: f64, t71403: f64, t71406: f64, t71408: f64, t71411: f64, t71414: f64, t71417: f64, t71420: f64, t71423: f64, t71426: f64, t50846: f64, t51151: f64, t71146: f64, t71150: f64, t71152: f64, t71154: f64, t71156: f64, t71160: f64, t71166: f64, t71170: f64, t71174: f64, t71179: f64, t71183: f64, t71187: f64, t71446: f64, t71449: f64, t71452: f64, t71454: f64, t71456: f64, t71458: f64, t71461: f64, t71463: f64, t71465: f64, t71191: f64, t71195: f64, t71199: f64, t71468: f64, t71470: f64, t71472: f64, t71474: f64, t71477: f64, t71480: f64, t71483: f64, t71486: f64, t71489: f64, t43859: f64, t44027: f64, t44053: f64, t50919: f64, t50948: f64, t71203: f64, t71206: f64, t71499: f64, t71501: f64, t71505: f64, t71508: f64, t71511: f64, t43816: f64, t51039: f64, t51051: f64, t63361: f64, t63382: f64, t63384: f64, t63398: f64, t63400: f64, t64074: f64, t64076: f64, t64087: f64, t64089: f64, t1099: f64, t1118: f64, t71558: f64, t21813: f64, t43964: f64, t11310: f64, t11361: f64, t11365: f64, t1148: f64, t1155: f64, t1156: f64, t1683: f64, t21907: f64, t21939: f64, t21942: f64, t21947: f64, t3371: f64, t44205: f64, t44220: f64, t4857: f64, t51371: f64, t51677: f64, t6069: f64, t6088: f64, t64254: f64, t71530: f64, t71543: f64, t71545: f64, t71547: f64, t21938: f64, t3403: f64, t1117: f64, t43969: f64, t21810: f64, t3264: f64, t21809: f64, t3315: f64, t3313: f64, t11275: f64, t18265: f64, t4781: f64, t21723: f64, t44075: f64, t44077: f64, t11415: f64, t15126: f64, t15136: f64, t1682: f64, t18603: f64, t18606: f64, t18622: f64, t18643: f64, t21845: f64, t21906: f64, t3357: f64, t3376: f64, t3401: f64, t43692: f64, t44155: f64, t44223: f64, t4819: f64, t63502: f64, t21886: f64, t3359: f64, t11350: f64, t1136: f64, t11420: f64, t15146: f64, t18609: f64, t18612: f64, t18616: f64, t18619: f64, t18647: f64, t18650: f64, t18651: f64, t21854: f64, t21855: f64, t21887: f64, t21890: f64, t3332: f64, t44177: f64, t44179: f64, t44361: f64, t4840: f64, t4862: f64, t51604: f64, t51680: f64, t6056: f64, t63454: f64, t63602: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t71571 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2549(t63332, t63334, t63336, t63886, t63888, t63893, t71124, t71130, t71135, t71140, t71142, t71391);
        let t71585 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2550(t63911, t71144, t71400, t71403, t71406, t71408, t71411, t71414, t71417, t71420, t71423, t71426);
        let t71597 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2551(t50846, t51151, t71146, t71150, t71152, t71154, t71156, t71160, t71166, t71170, t71174, t71179);
        let (t71611, t71624) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2552(t71183, t71187, t71446, t71449, t71452, t71454, t71456, t71458, t71461, t71463, t71465, t71191, t71195, t71199, t71468, t71470, t71472, t71474, t71477, t71480, t71483, t71486, t71489);
        let t71636 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2553(t43859, t44027, t44053, t50919, t50948, t71203, t71206, t71499, t71501, t71505, t71508, t71511);
        let t71649 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2554(t43816, t51039, t51051, t63361, t63382, t63384, t63398, t63400, t64074, t64076, t64087, t64089);
        let (t71655, t71657) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2555(t1099, t1118, t71558, t71571, t71585, t71597, t71611, t71624, t71636, t71649, t21813, t43964);
        let t71664 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2556(t11310, t11361, t11365, t1148, t1155, t1156, t1683, t21907, t21939, t21942, t21947, t3371, t44205, t44220, t4857, t51371, t51677, t6069, t6088, t64254, t71530, t71543, t71545, t71547, t71655, t71657);
        let (t71672, t71697, t71700, t71704, t71707) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2557(t21938, t3403, t1117, t21813, t43969, t21810, t3264, t21809, t3315, t3313, t11275, t18265, t4781);
        let (t71711, t71712) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2558(t1117, t21723, t44075, t44077, t11310, t11415, t1155, t15126, t15136, t1682, t18603, t18606, t18622, t18643, t21845, t21906, t21939, t21942, t3357, t3376, t3401, t43692, t44155, t44223, t4819, t4857, t63502, t71672, t71697, t71700, t71704, t71707);
        let t71752 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2559(t21886, t3359, t11350, t1136, t11420, t15126, t15136, t15146, t18609, t18612, t18616, t18619, t18647, t18650, t18651, t21854, t21855, t21887, t21890, t3332, t3357, t44177, t44179, t44361, t4819, t4840, t4862, t51604, t51680, t6056, t63454, t63602);
    (t71655, t71657, t71664, t71697, t71700, t71704, t71707, t71711, t71712, t71752)
}
