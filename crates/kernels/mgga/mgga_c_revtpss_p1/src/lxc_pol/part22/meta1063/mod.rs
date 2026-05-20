//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1063 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3804;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3805;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3806;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3807;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3808;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3809;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3810;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3811;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1063<F: Float>(t1298: F, t5501: F, t18134: F, t5023: F, t68700: F, t68703: F, t68707: F, t68709: F, t68711: F, t68714: F, t68716: F, t68718: F, t68723: F, t68725: F, t68727: F, t68730: F, t68733: F, t21639: F, t3794: F, t68735: F, t68738: F, t68742: F, t68744: F, t68746: F, t68748: F, t68751: F, t68754: F, t68757: F, t68760: F, t68763: F, t68766: F, t68769: F, t21635: F, t3801: F, t68772: F, t68779: F, t68781: F, t68784: F, t68786: F, t68789: F, t68791: F, t68794: F, t68799: F, t68803: F, t68805: F, t68808: F, t18123: F, t20692: F, t5505: F, t68942: F, t68946: F, t68949: F, t68951: F, t68954: F, t68956: F, t68959: F, t68961: F, t68963: F, t68965: F, t68967: F, t68969: F, t68971: F, t69030: F, t69090: F, t69094: F, t69097: F, t69099: F, t69101: F, t69103: F, t69105: F, t69107: F, t69111: F, t69115: F, t69117: F, t69599: F, t69569: F, t69571: F, t69573: F, t69575: F, t69577: F, t69579: F, t69581: F, t69583: F, t69585: F, t69587: F, t69590: F, t69594: F, t69603: F, t69605: F, t33: F, t265: F, t502: F, t63193: F, t68629: F, t73260: F, t1113: F, t1304: F, t13312: F, t1469: F, t15083: F, t1711: F, t18140: F, t18281: F, t1837: F, t18884: F, t20256: F, t21645: F, t2258: F, t2838: F, t3351: F, t3805: F, t4186: F, t504: F, t51835: F, t5509: F, t57: F, t5825: F, t606: F, t60754: F, t6084: F, t63202: F, t63204: F, t63206: F, t6416: F, t6757: F, t895: F, dens_threshold: F, rho1: F, zeta_threshold: F, t46279: F, t46281: F, t46286: F, t46302: F, t3857: F, t6801: F, t14304: F, t21969: F, t39419: F, t39422: F, t4139: F, t4140: F, t46289: F, t46297: F, t5541: F, t5542: F) -> (F, F, F, F, F, F, F) {
        let t73266 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3804::<F>(t1298, t5501, t18134, t5023, t68700, t68703, t68707, t68709, t68711, t68714, t68716, t68718, t68723, t68725, t68727, t68730, t68733);
        let t73270 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3805::<F>(t21639, t3794, t5023, t68735, t68738, t68742, t68744, t68746, t68748, t68751, t68754, t68757, t68760, t68763, t68766, t68769);
        let t73277 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3806::<F>(t21635, t3801, t1298, t5023, t68772, t68779, t68781, t68784, t68786, t68789, t68791, t68794, t68799, t68803, t68805, t68808);
        let t73283 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3807::<F>(t18123, t20692, t3794, t5023, t5505, t68942, t68946, t68949, t68951, t68954, t68956, t68959, t68961, t68963, t68965, t68967, t68969);
        let t73285 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3808::<F>(t68971, t69030, t69090, t69094, t69097, t69099, t69101, t69103, t69105, t69107, t69111, t69115, t69117, t69599);
        let t73286 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3809::<F>(t69569, t69571, t69573, t69575, t69577, t69579, t69581, t69583, t69585, t69587, t69590, t69594, t69603, t69605);
        let t73306 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3810::<F>(t33, t265, t502, t63193, t68629, t73260, t73266, t73270, t73277, t73283, t73285, t73286, t1113, t1304, t13312, t1469, t15083, t1711, t18140, t18281, t1837, t18884, t20256, t21645, t2258, t2838, t3351, t3805, t4186, t504, t51835, t5509, t57, t5825, t606, t60754, t6084, t63202, t63204, t63206, t6416, t6757, t895, dens_threshold, rho1, zeta_threshold);
        let (t73314, t73315, t73316, t73317, t73322, t73326) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3811::<F>(t46279, t46281, t46286, t46302, t3857, t6801, t14304, t21969, t39419, t39422, t4139, t4140, t46289, t46297, t5541, t5542);
    (t73306, t73314, t73315, t73316, t73317, t73322, t73326)
}
