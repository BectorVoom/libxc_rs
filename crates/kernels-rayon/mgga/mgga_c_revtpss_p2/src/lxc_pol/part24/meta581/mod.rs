//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta581 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1805;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1806;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1807;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1808;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1809;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1810;
use chunk6::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1811;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta581(t1300: f64, t198: f64, t336: f64, t89888: f64, t89930: f64, t90293: f64, t90321: f64, t90323: f64, t90327: f64, t90329: f64, t90332: f64, t90336: f64, t90339: f64, t90341: f64, t90343: f64, t90346: f64, t90349: f64, t91440: f64, t91748: f64, t1832: f64, t5023: f64, t81139: f64, t90351: f64, t90356: f64, t90361: f64, t90364: f64, t90367: f64, t90370: f64, t90373: f64, t90375: f64, t90377: f64, t90503: f64, t90505: f64, t90509: f64, t6752: f64, t44126: f64, t90511: f64, t90514: f64, t90578: f64, t90580: f64, t90582: f64, t90585: f64, t90588: f64, t90592: f64, t90594: f64, t90597: f64, t90599: f64, t90602: f64, t6748: f64, t3801: f64, t73252: f64, t90629: f64, t90631: f64, t90634: f64, t90636: f64, t90640: f64, t90644: f64, t90855: f64, t90857: f64, t90860: f64, t90863: f64, t90867: f64, t33: f64, t265: f64, t502: f64, t87990: f64, t1469: f64, t1587: f64, t1711: f64, t1837: f64, t22671: f64, t22783: f64, t23436: f64, t25032: f64, t504: f64, t57: f64, t5825: f64, t6084: f64, t6416: f64, t6757: f64, t87126: f64, t89780: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64, t30: f64, t6785: f64, t5824: f64, t1344: f64, t21944: f64, t22670: f64, t3874: f64, t46310: f64, t5574: f64, t87125: f64, t6792: f64, t1348: f64, t21956: f64, t3881: f64, t46328: f64, t5582: f64, t543: f64, t6816: f64, t6836: f64, t13804: f64, t1410: f64, t1868: f64, t22046: f64, t22079: f64, t22893: f64, t3934: f64, t3936: f64, t4003: f64, t4012: f64, t46627: f64, t5671: f64, t6869: f64, t73778: f64, t73789: f64, t828: f64, t85514: f64, t85516: f64, t85532: f64, t85543: f64, t85545: f64, t85553: f64, t85609: f64, t85648: f64, t85652: f64, t9955: f64, t9994: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t91754 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1805(t1300, t198, t336, t89888, t89930, t90293, t90321, t90323, t90327, t90329, t90332, t90336, t90339, t90341, t90343, t90346, t90349, t91440, t91748);
        let t91758 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1806(t1832, t5023, t81139, t90351, t90356, t90361, t90364, t90367, t90370, t90373, t90375, t90377, t90503, t90505, t90509);
        let t91765 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1807(t6752, t198, t336, t44126, t90511, t90514, t90578, t90580, t90582, t90585, t90588, t90592, t90594, t90597, t90599, t90602);
        let t91774 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1808(t6748, t198, t336, t3801, t5023, t6752, t73252, t90629, t90631, t90634, t90636, t90640, t90644, t90855, t90857, t90860, t90863, t90867);
        let t91789 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1809(t33, t265, t502, t87990, t91754, t91758, t91765, t91774, t1469, t1587, t1711, t1837, t22671, t22783, t23436, t25032, t504, t57, t5825, t6084, t6416, t6757, t87126, t89780, dens_threshold, rho1, zeta_threshold);
        let (t91797, t91802, t91810, t91811, t91816, t91824) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1810(t30, t33, t6785, t5824, t1344, t21944, t22670, t3874, t46310, t5574, t87125, t6792, t6416, t1348, t21956, t22783, t3881, t46328, t5582, t89780, zeta_threshold);
        let (t91826, t91865, t91870, t91875, t91882) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1811(t91810, t91824, t543, t6816, t6836, t13804, t1410, t1868, t22046, t22079, t22893, t3934, t3936, t4003, t4012, t46627, t5671, t6869, t73778, t73789, t828, t85514, t85516, t85532, t85543, t85545, t85553, t85609, t85648, t85652, t9955, t9994);
    (t91789, t91797, t91802, t91811, t91816, t91826, t91865, t91870, t91875, t91882)
}
