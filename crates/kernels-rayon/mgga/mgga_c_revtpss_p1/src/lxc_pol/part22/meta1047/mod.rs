//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1047 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3678;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3679;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3680;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3681;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1047(t12361: f64, t20577: f64, t20580: f64, t44101: f64, t20641: f64, t12243: f64, t20645: f64, t1149: f64, t20448: f64, t3384: f64, t20447: f64, t3435: f64, t3433: f64, t1170: f64, t1187: f64, t12486: f64, t12491: f64, t17150: f64, t1757: f64, t20537: f64, t20665: f64, t20668: f64, t20678: f64, t3496: f64, t3497: f64, t3515: f64, t45064: f64, t45177: f64, t6538: f64, t69565: f64, t69569: f64, t69571: f64, t69573: f64, t69575: f64, t69577: f64, t69579: f64, t300: f64, t69192: f64, t69216: f64, t69383: f64, t69422: f64, t69467: f64, t69500: f64, t69548: f64, t69090: f64, t69094: f64, t69097: f64, t69099: f64, t69101: f64, t69103: f64, t69105: f64, t69107: f64, t69111: f64, t69115: f64, t69117: f64, t1196: f64, t20895: f64, t3498: f64, t16673: f64, t5192: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t69581, t69583, t69585, t69587, t69590, t69591) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3678(t12361, t20577, t20580, t44101, t20641, t12243, t20645, t1149, t20448, t3384, t20447, t3435);
        let (t69594, t69595) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3679(t1149, t3433, t69591, t1170, t1187, t12486, t12491, t17150, t1757, t20537, t20665, t20668, t20678, t3496, t3497, t3515, t45064, t45177, t6538, t69565, t69569, t69571, t69573, t69575, t69577, t69579, t69581, t69583, t69585, t69587, t69590);
        let (t69599, t69600) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3680(t300, t69192, t69216, t69383, t69422, t69467, t69500, t69548, t69595, t69090, t69094, t69097, t69099, t69101, t69103, t69105, t69107, t69111, t69115, t69117, t69569);
        let (t69603, t69605, t69606) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3681(t1196, t20895, t3498, t16673, t5192, t69571, t69573, t69575, t69577, t69579, t69581, t69583, t69585, t69587, t69590, t69594);
    (t69581, t69583, t69585, t69587, t69590, t69594, t69599, t69600, t69603, t69605, t69606)
}
