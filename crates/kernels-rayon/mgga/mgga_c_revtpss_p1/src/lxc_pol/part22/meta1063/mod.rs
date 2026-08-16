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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3804;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3805;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3806;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3807;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3808;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3809;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3810;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3811;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1063(t1298: f64, t5501: f64, t18134: f64, t5023: f64, t68700: f64, t68703: f64, t68707: f64, t68709: f64, t68711: f64, t68714: f64, t68716: f64, t68718: f64, t68723: f64, t68725: f64, t68727: f64, t68730: f64, t68733: f64, t21639: f64, t3794: f64, t68735: f64, t68738: f64, t68742: f64, t68744: f64, t68746: f64, t68748: f64, t68751: f64, t68754: f64, t68757: f64, t68760: f64, t68763: f64, t68766: f64, t68769: f64, t21635: f64, t3801: f64, t68772: f64, t68779: f64, t68781: f64, t68784: f64, t68786: f64, t68789: f64, t68791: f64, t68794: f64, t68799: f64, t68803: f64, t68805: f64, t68808: f64, t18123: f64, t20692: f64, t5505: f64, t68942: f64, t68946: f64, t68949: f64, t68951: f64, t68954: f64, t68956: f64, t68959: f64, t68961: f64, t68963: f64, t68965: f64, t68967: f64, t68969: f64, t68971: f64, t69030: f64, t69090: f64, t69094: f64, t69097: f64, t69099: f64, t69101: f64, t69103: f64, t69105: f64, t69107: f64, t69111: f64, t69115: f64, t69117: f64, t69599: f64, t69569: f64, t69571: f64, t69573: f64, t69575: f64, t69577: f64, t69579: f64, t69581: f64, t69583: f64, t69585: f64, t69587: f64, t69590: f64, t69594: f64, t69603: f64, t69605: f64, t33: f64, t265: f64, t502: f64, t63193: f64, t68629: f64, t73260: f64, t1113: f64, t1304: f64, t13312: f64, t1469: f64, t15083: f64, t1711: f64, t18140: f64, t18281: f64, t1837: f64, t18884: f64, t20256: f64, t21645: f64, t2258: f64, t2838: f64, t3351: f64, t3805: f64, t4186: f64, t504: f64, t51835: f64, t5509: f64, t57: f64, t5825: f64, t606: f64, t60754: f64, t6084: f64, t63202: f64, t63204: f64, t63206: f64, t6416: f64, t6757: f64, t895: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64, t46279: f64, t46281: f64, t46286: f64, t46302: f64, t3857: f64, t6801: f64, t14304: f64, t21969: f64, t39419: f64, t39422: f64, t4139: f64, t4140: f64, t46289: f64, t46297: f64, t5541: f64, t5542: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let t73266 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3804(t1298, t5501, t18134, t5023, t68700, t68703, t68707, t68709, t68711, t68714, t68716, t68718, t68723, t68725, t68727, t68730, t68733);
        let t73270 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3805(t21639, t3794, t5023, t68735, t68738, t68742, t68744, t68746, t68748, t68751, t68754, t68757, t68760, t68763, t68766, t68769);
        let t73277 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3806(t21635, t3801, t1298, t5023, t68772, t68779, t68781, t68784, t68786, t68789, t68791, t68794, t68799, t68803, t68805, t68808);
        let t73283 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3807(t18123, t20692, t3794, t5023, t5505, t68942, t68946, t68949, t68951, t68954, t68956, t68959, t68961, t68963, t68965, t68967, t68969);
        let t73285 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3808(t68971, t69030, t69090, t69094, t69097, t69099, t69101, t69103, t69105, t69107, t69111, t69115, t69117, t69599);
        let t73286 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3809(t69569, t69571, t69573, t69575, t69577, t69579, t69581, t69583, t69585, t69587, t69590, t69594, t69603, t69605);
        let t73306 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3810(t33, t265, t502, t63193, t68629, t73260, t73266, t73270, t73277, t73283, t73285, t73286, t1113, t1304, t13312, t1469, t15083, t1711, t18140, t18281, t1837, t18884, t20256, t21645, t2258, t2838, t3351, t3805, t4186, t504, t51835, t5509, t57, t5825, t606, t60754, t6084, t63202, t63204, t63206, t6416, t6757, t895, dens_threshold, rho1, zeta_threshold);
        let (t73314, t73315, t73316, t73317, t73322, t73326) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3811(t46279, t46281, t46286, t46302, t3857, t6801, t14304, t21969, t39419, t39422, t4139, t4140, t46289, t46297, t5541, t5542);
    (t73306, t73314, t73315, t73316, t73317, t73322, t73326)
}
