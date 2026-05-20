//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta919 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2964;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2965;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2966;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2967;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2968;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta919<F: Float>(t1063: F, t11725: F, t23481: F, t247: F, t24031: F, t999: F, t23474: F, t3109: F, t23847: F, t3127: F, t3172: F, t22688: F, t23858: F, t23634: F, t1042: F, t11656: F, t11994: F, t15716: F, t15728: F, t15850: F, t1592: F, t15935: F, t1663: F, t19414: F, t19668: F, t19672: F, t19968: F, t23635: F, t23830: F, t23834: F, t23863: F, t23892: F, t3116: F, t42669: F, t42973: F, t4803: F, t4834: F, t54492: F, t54982: F, t6312: F, t6327: F, t65712: F, t1065: F, t1651: F, t4186: F, t4772: F, t6299: F, t1045: F, t11703: F, t11866: F, t1469: F, t15830: F, t15926: F, t16049: F, t16089: F, t16095: F, t18903: F, t18936: F, t18941: F, t19675: F, t19705: F, t19745: F, t19819: F, t23630: F, t23936: F, t23999: F, t2857: F, t3092: F, t3115: F, t3117: F, t3188: F, t4181: F, t42410: F, t4573: F, t4583: F, t4873: F, t4875: F, t4912: F, t55011: F, t55205: F, t6244: F, t6323: F, t65717: F, t65837: F, t67551: F, t78524: F, t906: F, t11256: F, t23642: F, t77492: F, t77494: F, t77496: F, t77498: F, t77600: F, t77604: F, t77612: F, t77622: F, t77624: F, t77628: F, t77634: F, t77636: F, t77639: F, t77641: F, t77643: F, t77645: F, t78402: F, t78405: F, t78411: F, t78413: F, t77647: F, t77657: F, t78417: F, t78422: F, t78426: F, t78428: F, t78432: F, t78435: F, t78438: F, t78441: F, t78443: F, t78446: F, t78449: F, t78451: F, t78456: F, t78458: F, t78460: F, t78463: F, t78465: F, t78469: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t78550, t78554, t78561, t78564, t78570) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2964::<F>(t1063, t11725, t23481, t247, t24031, t999, t23474, t3109, t23847, t3127, t3172, t22688);
        let t78601 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2965::<F>(t23858, t3127, t3172, t23634, t1042, t11656, t11994, t15716, t15728, t15850, t1592, t15935, t1663, t19414, t19668, t19672, t19968, t23635, t23830, t23834, t23863, t23892, t247, t3116, t42669, t42973, t4803, t4834, t54492, t54982, t6312, t6327, t65712, t78550, t78554, t78561, t78564, t78570);
        let (t78616, t78641, t78662) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2966::<F>(t1065, t24031, t1651, t4186, t4772, t6299, t1042, t1045, t11703, t11866, t1469, t15716, t15830, t15926, t16049, t16089, t16095, t18903, t18936, t18941, t19675, t19705, t19745, t19819, t23630, t23936, t23999, t2857, t3092, t3115, t3117, t3127, t3188, t4181, t42410, t4573, t4583, t4873, t4875, t4912, t55011, t55205, t6244, t6312, t6323, t65717, t65837, t67551, t78524, t906);
        let (t78676, t78682) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2967::<F>(t11256, t23642, t3172, t77492, t77494, t77496, t77498, t77600, t77604, t77612, t77622, t77624, t77628, t77634, t77636, t77639, t77641, t77643, t77645, t78402, t78405, t78411, t78413);
        let t78683 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2968::<F>(t77647, t77657, t78417, t78422, t78426, t78428, t78432, t78435, t78438, t78441, t78443, t78446, t78449, t78451, t78456, t78458, t78460, t78463, t78465, t78469);
    (t78554, t78570, t78601, t78616, t78641, t78662, t78676, t78682, t78683)
}
