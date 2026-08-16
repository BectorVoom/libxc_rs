//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1046 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3671;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3672;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3673;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3674;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3675;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3676;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3677;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1046<F: Float>(t58209: F, t58211: F, t58225: F, t68456: F, t68459: F, t68567: F, t68570: F, t68573: F, t68576: F, t68578: F, t68583: F, t68585: F, t68588: F, t68590: F, t68593: F, t1179: F, t20567: F, t3520: F, t6513: F, t5142: F, t3495: F, t3476: F, t6481: F, t1161: F, t1169: F, t1189: F, t17026: F, t17089: F, t1745: F, t1757: F, t20526: F, t20542: F, t3452: F, t3472: F, t3480: F, t3498: F, t3516: F, t3524: F, t45075: F, t5143: F, t5181: F, t58234: F, t58310: F, t6506: F, t68942: F, t68946: F, t68949: F, t68951: F, t68954: F, t69028: F, t69230: F, t69246: F, t69263: F, t69279: F, t69296: F, t69312: F, t69329: F, t20520: F, t3479: F, t1168: F, t12418: F, t12423: F, t12429: F, t12470: F, t12472: F, t12511: F, t17086: F, t20521: F, t20615: F, t20618: F, t20619: F, t20625: F, t3447: F, t3453: F, t3471: F, t3477: F, t45085: F, t45194: F, t5120: F, t6487: F, t6502: F, t6503: F, t68956: F, t68961: F, t68963: F, t68965: F, t68967: F, t1180: F, t1188: F, t16948: F, t16951: F, t16955: F, t16959: F, t16962: F, t17023: F, t17032: F, t17085: F, t20537: F, t20622: F, t20626: F, t3491: F, t3497: F, t3521: F, t3523: F, t45080: F, t45157: F, t45159: F, t45168: F, t45188: F, t45190: F, t5125: F, t5146: F, t5147: F, t58304: F, t58317: F, t58336: F, t6486: F, t6518: F, t6538: F, t68598: F, t68795: F, t69090: F, t3451: F, t12486: F, t16966: F, t20606: F, t20609: F, t20612: F, t20671: F, t3454: F, t3496: F, t3515: F, t45197: F, t58005: F, t6535: F, t69094: F, t69097: F, t69099: F, t69101: F, t69103: F, t69105: F, t69107: F, t20382: F, t12555: F, t6534: F, t1187: F, t12481: F, t12491: F, t12553: F, t16974: F, t16979: F, t16982: F, t16985: F, t16989: F, t16992: F, t16998: F, t17097: F, t17150: F, t17154: F, t20659: F, t20662: F, t20672: F, t20675: F, t5163: F, t5184: F, t5185: F, t58242: F, t58247: F, t58262: F, t58307: F, t6519: F, t1160: F, t20597: F, t16688: F, t16840: F, t5068: F, t58339: F, t5109: F, t58466: F, t16652: F, t17092: F, t16662: F, t12243: F, t20574: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t69345 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3671::<F>(t58209, t58211, t58225, t68456, t68459, t68567, t68570, t68573, t68576, t68578, t68583, t68585, t68588, t68590, t68593);
        let (t69367, t69383) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3672::<F>(t1179, t20567, t3520, t6513, t5142, t3495, t3476, t6481, t1161, t1169, t1189, t17026, t17089, t1745, t1757, t20526, t20542, t3452, t3472, t3480, t3498, t3516, t3524, t45075, t5143, t5181, t58234, t58310, t6506, t68942, t68946, t68949, t68951, t68954, t69028, t69230, t69246, t69263, t69279, t69296, t69312, t69329, t69345);
        let t69422 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3673::<F>(t20520, t3479, t1168, t12418, t12423, t12429, t12470, t12472, t12511, t17086, t20521, t20615, t20618, t20619, t20625, t3447, t3452, t3453, t3471, t3477, t45085, t45194, t5120, t6487, t6502, t6503, t6506, t68956, t68961, t68963, t68965, t68967);
        let t69467 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3674::<F>(t1180, t1188, t12423, t12470, t16948, t16951, t16955, t16959, t16962, t17023, t17032, t17085, t20537, t20622, t20625, t20626, t3453, t3471, t3477, t3491, t3497, t3521, t3523, t45080, t45157, t45159, t45168, t45188, t45190, t5125, t5146, t5147, t58304, t58317, t58336, t6486, t6518, t6538, t68598, t68795, t69090);
        let t69500 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3675::<F>(t3451, t6481, t1188, t12423, t12470, t12486, t12511, t16966, t17085, t1745, t20606, t20609, t20612, t20671, t3452, t3453, t3454, t3471, t3477, t3479, t3496, t3497, t3515, t45197, t58005, t6487, t6506, t6535, t68795, t69094, t69097, t69099, t69101, t69103, t69105, t69107, t69367);
        let t69548 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3676::<F>(t20382, t3523, t12555, t6534, t1187, t12481, t12491, t12553, t16974, t16979, t16982, t16985, t16989, t16992, t16998, t17032, t17097, t17150, t17154, t20659, t20662, t20671, t20672, t20675, t3497, t3515, t3521, t5163, t5184, t5185, t58242, t58247, t58262, t58307, t6519, t6538);
        let (t69565, t69569, t69571, t69573, t69575, t69577, t69579) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3677::<F>(t1160, t20597, t16688, t16840, t5068, t58339, t5109, t58466, t16652, t17092, t16662, t12243, t20574);
    (t69383, t69422, t69467, t69500, t69548, t69565, t69569, t69571, t69573, t69575, t69577, t69579)
}
