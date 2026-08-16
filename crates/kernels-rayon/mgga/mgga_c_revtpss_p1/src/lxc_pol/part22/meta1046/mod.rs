//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1046 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3671;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3672;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3673;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3674;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3675;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3676;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3677;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1046(t58209: f64, t58211: f64, t58225: f64, t68456: f64, t68459: f64, t68567: f64, t68570: f64, t68573: f64, t68576: f64, t68578: f64, t68583: f64, t68585: f64, t68588: f64, t68590: f64, t68593: f64, t1179: f64, t20567: f64, t3520: f64, t6513: f64, t5142: f64, t3495: f64, t3476: f64, t6481: f64, t1161: f64, t1169: f64, t1189: f64, t17026: f64, t17089: f64, t1745: f64, t1757: f64, t20526: f64, t20542: f64, t3452: f64, t3472: f64, t3480: f64, t3498: f64, t3516: f64, t3524: f64, t45075: f64, t5143: f64, t5181: f64, t58234: f64, t58310: f64, t6506: f64, t68942: f64, t68946: f64, t68949: f64, t68951: f64, t68954: f64, t69028: f64, t69230: f64, t69246: f64, t69263: f64, t69279: f64, t69296: f64, t69312: f64, t69329: f64, t20520: f64, t3479: f64, t1168: f64, t12418: f64, t12423: f64, t12429: f64, t12470: f64, t12472: f64, t12511: f64, t17086: f64, t20521: f64, t20615: f64, t20618: f64, t20619: f64, t20625: f64, t3447: f64, t3453: f64, t3471: f64, t3477: f64, t45085: f64, t45194: f64, t5120: f64, t6487: f64, t6502: f64, t6503: f64, t68956: f64, t68961: f64, t68963: f64, t68965: f64, t68967: f64, t1180: f64, t1188: f64, t16948: f64, t16951: f64, t16955: f64, t16959: f64, t16962: f64, t17023: f64, t17032: f64, t17085: f64, t20537: f64, t20622: f64, t20626: f64, t3491: f64, t3497: f64, t3521: f64, t3523: f64, t45080: f64, t45157: f64, t45159: f64, t45168: f64, t45188: f64, t45190: f64, t5125: f64, t5146: f64, t5147: f64, t58304: f64, t58317: f64, t58336: f64, t6486: f64, t6518: f64, t6538: f64, t68598: f64, t68795: f64, t69090: f64, t3451: f64, t12486: f64, t16966: f64, t20606: f64, t20609: f64, t20612: f64, t20671: f64, t3454: f64, t3496: f64, t3515: f64, t45197: f64, t58005: f64, t6535: f64, t69094: f64, t69097: f64, t69099: f64, t69101: f64, t69103: f64, t69105: f64, t69107: f64, t20382: f64, t12555: f64, t6534: f64, t1187: f64, t12481: f64, t12491: f64, t12553: f64, t16974: f64, t16979: f64, t16982: f64, t16985: f64, t16989: f64, t16992: f64, t16998: f64, t17097: f64, t17150: f64, t17154: f64, t20659: f64, t20662: f64, t20672: f64, t20675: f64, t5163: f64, t5184: f64, t5185: f64, t58242: f64, t58247: f64, t58262: f64, t58307: f64, t6519: f64, t1160: f64, t20597: f64, t16688: f64, t16840: f64, t5068: f64, t58339: f64, t5109: f64, t58466: f64, t16652: f64, t17092: f64, t16662: f64, t12243: f64, t20574: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t69345 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3671(t58209, t58211, t58225, t68456, t68459, t68567, t68570, t68573, t68576, t68578, t68583, t68585, t68588, t68590, t68593);
        let (t69367, t69383) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3672(t1179, t20567, t3520, t6513, t5142, t3495, t3476, t6481, t1161, t1169, t1189, t17026, t17089, t1745, t1757, t20526, t20542, t3452, t3472, t3480, t3498, t3516, t3524, t45075, t5143, t5181, t58234, t58310, t6506, t68942, t68946, t68949, t68951, t68954, t69028, t69230, t69246, t69263, t69279, t69296, t69312, t69329, t69345);
        let t69422 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3673(t20520, t3479, t1168, t12418, t12423, t12429, t12470, t12472, t12511, t17086, t20521, t20615, t20618, t20619, t20625, t3447, t3452, t3453, t3471, t3477, t45085, t45194, t5120, t6487, t6502, t6503, t6506, t68956, t68961, t68963, t68965, t68967);
        let t69467 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3674(t1180, t1188, t12423, t12470, t16948, t16951, t16955, t16959, t16962, t17023, t17032, t17085, t20537, t20622, t20625, t20626, t3453, t3471, t3477, t3491, t3497, t3521, t3523, t45080, t45157, t45159, t45168, t45188, t45190, t5125, t5146, t5147, t58304, t58317, t58336, t6486, t6518, t6538, t68598, t68795, t69090);
        let t69500 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3675(t3451, t6481, t1188, t12423, t12470, t12486, t12511, t16966, t17085, t1745, t20606, t20609, t20612, t20671, t3452, t3453, t3454, t3471, t3477, t3479, t3496, t3497, t3515, t45197, t58005, t6487, t6506, t6535, t68795, t69094, t69097, t69099, t69101, t69103, t69105, t69107, t69367);
        let t69548 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3676(t20382, t3523, t12555, t6534, t1187, t12481, t12491, t12553, t16974, t16979, t16982, t16985, t16989, t16992, t16998, t17032, t17097, t17150, t17154, t20659, t20662, t20671, t20672, t20675, t3497, t3515, t3521, t5163, t5184, t5185, t58242, t58247, t58262, t58307, t6519, t6538);
        let (t69565, t69569, t69571, t69573, t69575, t69577, t69579) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3677(t1160, t20597, t16688, t16840, t5068, t58339, t5109, t58466, t16652, t17092, t16662, t12243, t20574);
    (t69383, t69422, t69467, t69500, t69548, t69565, t69569, t69571, t69573, t69575, t69577, t69579)
}
