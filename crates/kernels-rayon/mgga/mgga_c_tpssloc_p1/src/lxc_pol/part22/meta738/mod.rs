//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta738 (260520-c91 hierarchical CSE).
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
mod chunk11;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2422;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2423;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2424;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2425;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2426;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2427;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2428;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2429;
use chunk8::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2430;
use chunk9::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2431;
use chunk10::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2432;
use chunk11::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2433;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta738(t17517: f64, t49226: f64, t21347: f64, t942: f64, t10765: f64, t14266: f64, t1569: f64, t17428: f64, t21259: f64, t4434: f64, t49427: f64, t5743: f64, t5759: f64, t59962: f64, t68762: f64, t68764: f64, t68767: f64, t68769: f64, t68771: f64, t68773: f64, t68775: f64, t68883: f64, t68885: f64, t952: f64, t48103: f64, t49304: f64, t49306: f64, t49317: f64, t49322: f64, t68442: f64, t68444: f64, t68446: f64, t68448: f64, t68452: f64, t68454: f64, t41684: f64, t41863: f64, t68460: f64, t68464: f64, t68468: f64, t68472: f64, t68479: f64, t68483: f64, t68486: f64, t68489: f64, t68492: f64, t68494: f64, t68498: f64, t68500: f64, t68502: f64, t68504: f64, t68506: f64, t68509: f64, t68511: f64, t68515: f64, t68518: f64, t68523: f64, t68527: f64, t68530: f64, t49378: f64, t59657: f64, t60163: f64, t60168: f64, t60173: f64, t68536: f64, t68541: f64, t68545: f64, t68549: f64, t68552: f64, t68556: f64, t68563: f64, t49379: f64, t60192: f64, t60194: f64, t60202: f64, t68571: f64, t68577: f64, t68580: f64, t68583: f64, t68586: f64, t68589: f64, t68592: f64, t42212: f64, t59663: f64, t59665: f64, t59680: f64, t59688: f64, t59694: f64, t60204: f64, t68596: f64, t68599: f64, t68602: f64, t68605: f64, t68608: f64, t42213: f64, t47787: f64, t59700: f64, t59702: f64, t59704: f64, t60274: f64, t68619: f64, t68626: f64, t68628: f64, t68630: f64, t68633: f64, t68635: f64, t59759: f64, t59761: f64, t60308: f64, t60310: f64, t60312: f64, t68638: f64, t68640: f64, t68643: f64, t68646: f64, t68649: f64, t68695: f64, t68697: f64, t10756: f64, t10825: f64, t14332: f64, t14369: f64, t1581: f64, t17350: f64, t17355: f64, t21115: f64, t21195: f64, t21198: f64, t21247: f64, t2856: f64, t41984: f64, t42149: f64, t4411: f64, t4472: f64, t48789: f64, t49096: f64, t5762: f64, t5775: f64, t5790: f64, t5791: f64, t60338: f64, t68758: f64, t68926: f64, t68995: f64, t924: f64, t932: f64, t950: f64, t21360: f64, t923: f64, t10828: f64, t14263: f64, t14337: f64, t1568: f64, t17443: f64, t17446: f64, t17451: f64, t17499: f64, t17547: f64, t21089: f64, t21207: f64, t21242: f64, t21306: f64, t2886: f64, t2930: f64, t41826: f64, t42111: f64, t42113: f64, t4433: f64, t4471: f64, t49099: f64, t60775: f64, t69003: f64, t69005: f64, t933: f64, t10740: f64, t10747: f64, t10771: f64, t1580: f64, t17297: f64, t17349: f64, t17454: f64, t17493: f64, t17544: f64, t21309: f64, t21312: f64, t21321: f64, t2861: f64, t2905: f64, t4438: f64, t48783: f64, t49263: f64, t49422: f64, t5758: f64, t59895: f64, t69011: f64, t69018: f64, t4359: f64, t60357: f64, t4400: f64, t59959: f64, t13727: f64, t13520: f64, t17521: f64, t17524: f64, t17528: f64, t49274: f64, t21238: f64, t2932: f64, t17496: f64, t17500: f64, t21239: f64, t4454: f64, t4476: f64, t49104: f64, t5794: f64, t60343: f64, t60424: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t69036, t69050) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2422(t17517, t49226, t21347, t942, t10765, t14266, t1569, t17428, t21259, t4434, t49427, t5743, t5759, t59962, t68762, t68764, t68767, t68769, t68771, t68773, t68775, t68883, t68885, t952);
        let (t69066, t69079) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2423(t48103, t49304, t49306, t49317, t49322, t68442, t68444, t68446, t68448, t68452, t68454, t41684, t41863, t68460, t68464, t68468, t68472, t68479, t68483, t68486, t68489, t68492, t68494);
        let t69093 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2424(t68498, t68500, t68502, t68504, t68506, t68509, t68511, t68515, t68518, t68523, t68527, t68530);
        let t69105 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2425(t49378, t59657, t60163, t60168, t60173, t68536, t68541, t68545, t68549, t68552, t68556, t68563);
        let (t69118, t69130) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2426(t49379, t60192, t60194, t60202, t68571, t68577, t68580, t68583, t68586, t68589, t68592, t42212, t59663, t59665, t59680, t59688, t59694, t60204, t68596, t68599, t68602, t68605, t68608);
        let t69143 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2427(t42213, t47787, t59700, t59702, t59704, t60274, t68619, t68626, t68628, t68630, t68633, t68635);
        let t69156 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2428(t59759, t59761, t60308, t60310, t60312, t68638, t68640, t68643, t68646, t68649, t68695, t68697);
        let t69180 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2429(t10756, t10825, t14332, t14369, t1581, t17350, t17355, t21115, t21195, t21198, t21247, t2856, t41984, t42149, t4411, t4472, t48789, t49096, t5762, t5775, t5790, t5791, t60338, t68758, t68926, t68995, t69066, t69079, t69093, t69105, t69118, t69130, t69143, t69156, t924, t932, t950);
        let t69218 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2430(t21360, t923, t10756, t10765, t10828, t14263, t14337, t1568, t17443, t17446, t17451, t17499, t17547, t21089, t21207, t21242, t21247, t21306, t2886, t2930, t41826, t42111, t42113, t4433, t4471, t49099, t5775, t60775, t69003, t69005, t933, t950);
        let t69249 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2431(t10740, t10747, t10771, t10825, t1569, t1580, t1581, t17297, t17349, t17454, t17493, t17544, t21309, t21312, t21321, t2861, t2905, t2930, t4434, t4438, t4472, t48783, t49263, t49422, t5758, t5790, t59895, t69011, t69018, t69036);
        let (t69253, t69255, t69257, t69259, t69261, t69263, t69276) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2432(t4359, t60357, t4400, t59959, t13727, t17517, t13520, t17521, t17524, t17528, t49274, t21238, t2932);
        let t69286 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2433(t10828, t14263, t14337, t17454, t17493, t17496, t17500, t21239, t2905, t2930, t4454, t4471, t4476, t49104, t5794, t60343, t60424, t69253, t69255, t69257, t69259, t69261, t69263, t69276, t950);
    (t69036, t69050, t69180, t69218, t69249, t69253, t69255, t69257, t69259, t69261, t69263, t69286)
}
