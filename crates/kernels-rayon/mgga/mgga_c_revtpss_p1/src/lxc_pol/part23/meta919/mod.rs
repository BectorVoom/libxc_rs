//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta919 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2964;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2965;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2966;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2967;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2968;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta919(t1063: f64, t11725: f64, t23481: f64, t247: f64, t24031: f64, t999: f64, t23474: f64, t3109: f64, t23847: f64, t3127: f64, t3172: f64, t22688: f64, t23858: f64, t23634: f64, t1042: f64, t11656: f64, t11994: f64, t15716: f64, t15728: f64, t15850: f64, t1592: f64, t15935: f64, t1663: f64, t19414: f64, t19668: f64, t19672: f64, t19968: f64, t23635: f64, t23830: f64, t23834: f64, t23863: f64, t23892: f64, t3116: f64, t42669: f64, t42973: f64, t4803: f64, t4834: f64, t54492: f64, t54982: f64, t6312: f64, t6327: f64, t65712: f64, t1065: f64, t1651: f64, t4186: f64, t4772: f64, t6299: f64, t1045: f64, t11703: f64, t11866: f64, t1469: f64, t15830: f64, t15926: f64, t16049: f64, t16089: f64, t16095: f64, t18903: f64, t18936: f64, t18941: f64, t19675: f64, t19705: f64, t19745: f64, t19819: f64, t23630: f64, t23936: f64, t23999: f64, t2857: f64, t3092: f64, t3115: f64, t3117: f64, t3188: f64, t4181: f64, t42410: f64, t4573: f64, t4583: f64, t4873: f64, t4875: f64, t4912: f64, t55011: f64, t55205: f64, t6244: f64, t6323: f64, t65717: f64, t65837: f64, t67551: f64, t78524: f64, t906: f64, t11256: f64, t23642: f64, t77492: f64, t77494: f64, t77496: f64, t77498: f64, t77600: f64, t77604: f64, t77612: f64, t77622: f64, t77624: f64, t77628: f64, t77634: f64, t77636: f64, t77639: f64, t77641: f64, t77643: f64, t77645: f64, t78402: f64, t78405: f64, t78411: f64, t78413: f64, t77647: f64, t77657: f64, t78417: f64, t78422: f64, t78426: f64, t78428: f64, t78432: f64, t78435: f64, t78438: f64, t78441: f64, t78443: f64, t78446: f64, t78449: f64, t78451: f64, t78456: f64, t78458: f64, t78460: f64, t78463: f64, t78465: f64, t78469: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t78550, t78554, t78561, t78564, t78570) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2964(t1063, t11725, t23481, t247, t24031, t999, t23474, t3109, t23847, t3127, t3172, t22688);
        let t78601 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2965(t23858, t3127, t3172, t23634, t1042, t11656, t11994, t15716, t15728, t15850, t1592, t15935, t1663, t19414, t19668, t19672, t19968, t23635, t23830, t23834, t23863, t23892, t247, t3116, t42669, t42973, t4803, t4834, t54492, t54982, t6312, t6327, t65712, t78550, t78554, t78561, t78564, t78570);
        let (t78616, t78641, t78662) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2966(t1065, t24031, t1651, t4186, t4772, t6299, t1042, t1045, t11703, t11866, t1469, t15716, t15830, t15926, t16049, t16089, t16095, t18903, t18936, t18941, t19675, t19705, t19745, t19819, t23630, t23936, t23999, t2857, t3092, t3115, t3117, t3127, t3188, t4181, t42410, t4573, t4583, t4873, t4875, t4912, t55011, t55205, t6244, t6312, t6323, t65717, t65837, t67551, t78524, t906);
        let (t78676, t78682) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2967(t11256, t23642, t3172, t77492, t77494, t77496, t77498, t77600, t77604, t77612, t77622, t77624, t77628, t77634, t77636, t77639, t77641, t77643, t77645, t78402, t78405, t78411, t78413);
        let t78683 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2968(t77647, t77657, t78417, t78422, t78426, t78428, t78432, t78435, t78438, t78441, t78443, t78446, t78449, t78451, t78456, t78458, t78460, t78463, t78465, t78469);
    (t78554, t78570, t78601, t78616, t78641, t78662, t78676, t78682, t78683)
}
