//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1047 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3678;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3679;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3680;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3681;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1047<F: Float>(t12361: F, t20577: F, t20580: F, t44101: F, t20641: F, t12243: F, t20645: F, t1149: F, t20448: F, t3384: F, t20447: F, t3435: F, t3433: F, t1170: F, t1187: F, t12486: F, t12491: F, t17150: F, t1757: F, t20537: F, t20665: F, t20668: F, t20678: F, t3496: F, t3497: F, t3515: F, t45064: F, t45177: F, t6538: F, t69565: F, t69569: F, t69571: F, t69573: F, t69575: F, t69577: F, t69579: F, t300: F, t69192: F, t69216: F, t69383: F, t69422: F, t69467: F, t69500: F, t69548: F, t69090: F, t69094: F, t69097: F, t69099: F, t69101: F, t69103: F, t69105: F, t69107: F, t69111: F, t69115: F, t69117: F, t1196: F, t20895: F, t3498: F, t16673: F, t5192: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t69581, t69583, t69585, t69587, t69590, t69591) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3678::<F>(t12361, t20577, t20580, t44101, t20641, t12243, t20645, t1149, t20448, t3384, t20447, t3435);
        let (t69594, t69595) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3679::<F>(t1149, t3433, t69591, t1170, t1187, t12486, t12491, t17150, t1757, t20537, t20665, t20668, t20678, t3496, t3497, t3515, t45064, t45177, t6538, t69565, t69569, t69571, t69573, t69575, t69577, t69579, t69581, t69583, t69585, t69587, t69590);
        let (t69599, t69600) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3680::<F>(t300, t69192, t69216, t69383, t69422, t69467, t69500, t69548, t69595, t69090, t69094, t69097, t69099, t69101, t69103, t69105, t69107, t69111, t69115, t69117, t69569);
        let (t69603, t69605, t69606) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3681::<F>(t1196, t20895, t3498, t16673, t5192, t69571, t69573, t69575, t69577, t69579, t69581, t69583, t69585, t69587, t69590, t69594);
    (t69581, t69583, t69585, t69587, t69590, t69594, t69599, t69600, t69603, t69605, t69606)
}
