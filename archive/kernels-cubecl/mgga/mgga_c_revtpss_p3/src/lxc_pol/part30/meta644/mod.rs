//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta644 (260520-c91 hierarchical CSE).
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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2261;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2262;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2263;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2264;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2265;
use chunk5::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2266;
use chunk6::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2267;
use chunk7::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2268;
use chunk8::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2269;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta644<F: Float>(t104416: F, t1519: F, t1911: F, t2372: F, t27060: F, t27066: F, t29427: F, t4257: F, t96706: F, t98559: F, t98562: F, t98567: F, t98569: F, t98571: F, t98574: F, t98578: F, t98581: F, t98584: F, t98590: F, t98594: F, t98597: F, t98599: F, t98601: F, t101124: F, t101416: F, t101420: F, t101422: F, t101428: F, t101431: F, t101436: F, t104163: F, t104408: F, t1310: F, t29422: F, t508: F, t98603: F, t98605: F, t98607: F, t98609: F, t98611: F, t98615: F, t98617: F, t98621: F, t98623: F, t101439: F, t101472: F, t101476: F, t101482: F, t101485: F, t101486: F, t101546: F, t101548: F, t101550: F, t101552: F, t13521: F, t13532: F, t14310: F, t1843: F, t2165: F, t26800: F, t26804: F, t3813: F, t4151: F, t5517: F, t5787: F, t7584: F, t7586: F, t7687: F, t8152: F, t8237: F, t104135: F, t104153: F, t104433: F, t105712: F, t105724: F, t101568: F, t101570: F, t101572: F, t101576: F, t101578: F, t101583: F, t101586: F, t101590: F, t101594: F, t101598: F, t101601: F, t101606: F, t18204: F, t18211: F, t2170: F, t4165: F, t573: F, t5805: F, t7696: F, t8245: F, param_d: F, t101613: F, t101617: F, t101619: F, t101621: F, t101625: F, t101628: F, t101632: F, t101634: F, t101640: F, t101642: F, t101645: F, t101648: F, t1461: F, t18208: F, t18214: F, t1918: F, t27102: F, t29480: F, t4162: F, t5802: F, t104094: F, t1456: F, t1458: F, t1464: F, t18178: F, t1914: F, t1921: F, t2172: F, t27090: F, t27110: F, t29469: F, t29490: F, t3: F, t4154: F, t4168: F, t575: F, t5790: F, t5808: F, t7691: F, t7700: F, t8241: F, t8249: F, t96690: F, t29468: F, t8240: F, t7690: F, t2167: F, t5789: F, t1913: F, t571: F, t18217: F, t2168: F, t96684: F, t96692: F, t96694: F, t97580: F, t97586: F) -> F {
        let t105734 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2261::<F>(t104416, t1519, t1911, t2372, t27060, t27066, t29427, t4257, t96706, t98559, t98562, t98567, t98569, t98571, t98574, t98578, t98581, t98584, t98590, t98594, t98597, t98599, t98601);
        let t105741 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2262::<F>(t101124, t101416, t101420, t101422, t101428, t101431, t101436, t104163, t104408, t1310, t29422, t508, t98603, t98605, t98607, t98609, t98611, t98615, t98617, t98621, t98623);
        let t105756 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2263::<F>(t101439, t101472, t101476, t101482, t101485, t101486, t101546, t101548, t101550, t101552, t13521, t13532, t14310, t1843, t2165, t26800, t26804, t3813, t4151, t5517, t5787, t7584, t7586, t7687, t8152, t8237);
        let (t105759, t105762) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2264::<F>(t104135, t104153, t104433, t105712, t105724, t105734, t105741, t105756, t101568, t101570, t101572, t101576, t101578, t101583, t101586, t101590, t101594, t101598, t101601, t101606, t18204, t18211, t2170, t4165, t573, t5805, t7696, t8245, param_d);
        let t105775 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2265::<F>(t101613, t101617, t101619, t101621, t101625, t101628, t101632, t101634, t101640, t101642, t101645, t101648, t1461, t18208, t18214, t1918, t2170, t27102, t29480, t4162, t5802, t7696, t8245);
        let t105789 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2266::<F>(t104094, t105759, t105762, t105775, t1456, t1458, t1464, t18178, t1914, t1921, t2172, t27090, t27110, t29469, t29490, t3, t4154, t4168, t575, t5790, t5808, t7691, t7700, t8241, t8249, t96690);
        let (t105792, t105794, t105796, t105798, t105800, t105802) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2267::<F>(t29468, t575, t1464, t8240, t1921, t7690, t2167, t5808, t2172, t5789, t1913, t7700);
        let t105806 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2268::<F>(t29490, t571, t105792, t105794, t105796, t105798, t105800, t105802, t18217, t2168, t96684, t96692, t96694, t97580, t97586);
        let tv4rho3sigma5 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2269::<F>(t105789, t105806);
    tv4rho3sigma5
}
