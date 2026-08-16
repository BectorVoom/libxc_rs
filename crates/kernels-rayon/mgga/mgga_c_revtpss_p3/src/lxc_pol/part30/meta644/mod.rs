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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta644(t104416: f64, t1519: f64, t1911: f64, t2372: f64, t27060: f64, t27066: f64, t29427: f64, t4257: f64, t96706: f64, t98559: f64, t98562: f64, t98567: f64, t98569: f64, t98571: f64, t98574: f64, t98578: f64, t98581: f64, t98584: f64, t98590: f64, t98594: f64, t98597: f64, t98599: f64, t98601: f64, t101124: f64, t101416: f64, t101420: f64, t101422: f64, t101428: f64, t101431: f64, t101436: f64, t104163: f64, t104408: f64, t1310: f64, t29422: f64, t508: f64, t98603: f64, t98605: f64, t98607: f64, t98609: f64, t98611: f64, t98615: f64, t98617: f64, t98621: f64, t98623: f64, t101439: f64, t101472: f64, t101476: f64, t101482: f64, t101485: f64, t101486: f64, t101546: f64, t101548: f64, t101550: f64, t101552: f64, t13521: f64, t13532: f64, t14310: f64, t1843: f64, t2165: f64, t26800: f64, t26804: f64, t3813: f64, t4151: f64, t5517: f64, t5787: f64, t7584: f64, t7586: f64, t7687: f64, t8152: f64, t8237: f64, t104135: f64, t104153: f64, t104433: f64, t105712: f64, t105724: f64, t101568: f64, t101570: f64, t101572: f64, t101576: f64, t101578: f64, t101583: f64, t101586: f64, t101590: f64, t101594: f64, t101598: f64, t101601: f64, t101606: f64, t18204: f64, t18211: f64, t2170: f64, t4165: f64, t573: f64, t5805: f64, t7696: f64, t8245: f64, param_d: f64, t101613: f64, t101617: f64, t101619: f64, t101621: f64, t101625: f64, t101628: f64, t101632: f64, t101634: f64, t101640: f64, t101642: f64, t101645: f64, t101648: f64, t1461: f64, t18208: f64, t18214: f64, t1918: f64, t27102: f64, t29480: f64, t4162: f64, t5802: f64, t104094: f64, t1456: f64, t1458: f64, t1464: f64, t18178: f64, t1914: f64, t1921: f64, t2172: f64, t27090: f64, t27110: f64, t29469: f64, t29490: f64, t3: f64, t4154: f64, t4168: f64, t575: f64, t5790: f64, t5808: f64, t7691: f64, t7700: f64, t8241: f64, t8249: f64, t96690: f64, t29468: f64, t8240: f64, t7690: f64, t2167: f64, t5789: f64, t1913: f64, t571: f64, t18217: f64, t2168: f64, t96684: f64, t96692: f64, t96694: f64, t97580: f64, t97586: f64) -> f64 {
        let t105734 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2261(t104416, t1519, t1911, t2372, t27060, t27066, t29427, t4257, t96706, t98559, t98562, t98567, t98569, t98571, t98574, t98578, t98581, t98584, t98590, t98594, t98597, t98599, t98601);
        let t105741 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2262(t101124, t101416, t101420, t101422, t101428, t101431, t101436, t104163, t104408, t1310, t29422, t508, t98603, t98605, t98607, t98609, t98611, t98615, t98617, t98621, t98623);
        let t105756 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2263(t101439, t101472, t101476, t101482, t101485, t101486, t101546, t101548, t101550, t101552, t13521, t13532, t14310, t1843, t2165, t26800, t26804, t3813, t4151, t5517, t5787, t7584, t7586, t7687, t8152, t8237);
        let (t105759, t105762) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2264(t104135, t104153, t104433, t105712, t105724, t105734, t105741, t105756, t101568, t101570, t101572, t101576, t101578, t101583, t101586, t101590, t101594, t101598, t101601, t101606, t18204, t18211, t2170, t4165, t573, t5805, t7696, t8245, param_d);
        let t105775 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2265(t101613, t101617, t101619, t101621, t101625, t101628, t101632, t101634, t101640, t101642, t101645, t101648, t1461, t18208, t18214, t1918, t2170, t27102, t29480, t4162, t5802, t7696, t8245);
        let t105789 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2266(t104094, t105759, t105762, t105775, t1456, t1458, t1464, t18178, t1914, t1921, t2172, t27090, t27110, t29469, t29490, t3, t4154, t4168, t575, t5790, t5808, t7691, t7700, t8241, t8249, t96690);
        let (t105792, t105794, t105796, t105798, t105800, t105802) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2267(t29468, t575, t1464, t8240, t1921, t7690, t2167, t5808, t2172, t5789, t1913, t7700);
        let t105806 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2268(t29490, t571, t105792, t105794, t105796, t105798, t105800, t105802, t18217, t2168, t96684, t96692, t96694, t97580, t97586);
        let tv4rho3sigma5 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2269(t105789, t105806);
    tv4rho3sigma5
}
