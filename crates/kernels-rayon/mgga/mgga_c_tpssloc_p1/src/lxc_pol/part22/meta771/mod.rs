//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta771 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2625;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2626;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2627;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2628;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2629;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2630;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2631;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta771(t1174: f64, t22059: f64, t3431: f64, t50846: f64, t63841: f64, t63843: f64, t63845: f64, t63886: f64, t63888: f64, t63893: f64, t63911: f64, t71333: f64, t71335: f64, t71337: f64, t71400: f64, t71406: f64, t71408: f64, t71411: f64, t71414: f64, t71417: f64, t71420: f64, t71423: f64, t71426: f64, t43859: f64, t44466: f64, t52313: f64, t52339: f64, t52343: f64, t64074: f64, t64076: f64, t64087: f64, t64089: f64, t71470: f64, t71472: f64, t71474: f64, t71477: f64, t71480: f64, t71483: f64, t71486: f64, t71489: f64, t71505: f64, t71508: f64, t71511: f64, t18529: f64, t4889: f64, t135: f64, t22034: f64, t15338: f64, t18409: f64, t3447: f64, t15320: f64, t15376: f64, t18427: f64, t18434: f64, t52058: f64, t64711: f64, t64713: f64, t64718: f64, t64730: f64, t64733: f64, t20217: f64, t3450: f64, t18469: f64, t52059: f64, t4904: f64, t64763: f64, t18532: f64, t22040: f64, t18321: f64, t4916: f64, t1187: f64, t18437: f64, t18526: f64, t3449: f64, t4908: f64, t4931: f64, t52074: f64, t52081: f64, t52085: f64, t64765: f64, t64770: f64, t64773: f64, t64781: f64, t64784: f64, t64821: f64, t71177: f64, t73113: f64, t11583: f64, t21510: f64, t11570: f64, t15382: f64, t18484: f64, t44478: f64, t4919: f64, t5979: f64, t64648: f64, t64951: f64, t64969: f64, t64976: f64, t64979: f64, t64981: f64, t64988: f64, t65077: f64, t7319: f64, t15419: f64, t21745: f64, t20234: f64, t44505: f64, t1171: f64, t22104: f64, t15313: f64, t18416: f64, t4920: f64, t64756: f64, t64775: f64, t64811: f64, t65035: f64, t65041: f64, t65093: f64, t65112: f64, t65126: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t73330, t73355) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2625(t1174, t22059, t3431, t50846, t63841, t63843, t63845, t63886, t63888, t63893, t63911, t71333, t71335, t71337, t71400, t71406, t71408, t71411, t71414, t71417, t71420, t71423, t71426);
        let t73369 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2626(t43859, t44466, t52313, t52339, t52343, t64074, t64076, t64087, t64089, t71470, t71472, t71474, t71477, t71480, t71483, t71486, t71489, t71505, t71508, t71511);
        let t73399 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2627(t18529, t4889, t1174, t135, t22034, t15338, t18409, t3447, t15320, t15376, t18427, t18434, t52058, t64711, t64713, t64718, t64730, t64733);
        let (t73405, t73417, t73420, t73424, t73427) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2628(t20217, t3450, t18469, t3447, t52059, t4904, t64763, t18532, t4889, t1174, t135, t22040);
        let t73439 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2629(t18321, t4916, t1187, t15376, t18437, t18526, t3447, t3449, t4889, t4908, t4931, t52074, t52081, t52085, t64765, t64770, t64773, t64781, t64784, t64821, t71177, t73113, t73405, t73417, t73420, t73424, t73427);
        let (t73444, t73451, t73480) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2630(t11583, t21510, t11570, t15376, t15382, t18484, t3447, t44478, t4919, t5979, t64648, t64951, t64969, t64976, t64979, t64981, t64988, t65077, t7319);
        let (t73491, t73496, t73525) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2631(t15419, t21745, t3447, t20234, t44505, t1171, t22104, t15313, t15320, t18409, t18416, t4904, t4919, t4920, t64756, t64775, t64811, t65035, t65041, t65093, t65112, t65126);
    (t73330, t73355, t73369, t73399, t73439, t73444, t73451, t73480, t73491, t73496, t73525)
}
