//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta581 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1805;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1806;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1807;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1808;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1809;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1810;
use chunk6::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1811;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta581<F: Float>(t1300: F, t198: F, t336: F, t89888: F, t89930: F, t90293: F, t90321: F, t90323: F, t90327: F, t90329: F, t90332: F, t90336: F, t90339: F, t90341: F, t90343: F, t90346: F, t90349: F, t91440: F, t91748: F, t1832: F, t5023: F, t81139: F, t90351: F, t90356: F, t90361: F, t90364: F, t90367: F, t90370: F, t90373: F, t90375: F, t90377: F, t90503: F, t90505: F, t90509: F, t6752: F, t44126: F, t90511: F, t90514: F, t90578: F, t90580: F, t90582: F, t90585: F, t90588: F, t90592: F, t90594: F, t90597: F, t90599: F, t90602: F, t6748: F, t3801: F, t73252: F, t90629: F, t90631: F, t90634: F, t90636: F, t90640: F, t90644: F, t90855: F, t90857: F, t90860: F, t90863: F, t90867: F, t33: F, t265: F, t502: F, t87990: F, t1469: F, t1587: F, t1711: F, t1837: F, t22671: F, t22783: F, t23436: F, t25032: F, t504: F, t57: F, t5825: F, t6084: F, t6416: F, t6757: F, t87126: F, t89780: F, dens_threshold: F, rho1: F, zeta_threshold: F, t30: F, t6785: F, t5824: F, t1344: F, t21944: F, t22670: F, t3874: F, t46310: F, t5574: F, t87125: F, t6792: F, t1348: F, t21956: F, t3881: F, t46328: F, t5582: F, t543: F, t6816: F, t6836: F, t13804: F, t1410: F, t1868: F, t22046: F, t22079: F, t22893: F, t3934: F, t3936: F, t4003: F, t4012: F, t46627: F, t5671: F, t6869: F, t73778: F, t73789: F, t828: F, t85514: F, t85516: F, t85532: F, t85543: F, t85545: F, t85553: F, t85609: F, t85648: F, t85652: F, t9955: F, t9994: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t91754 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1805::<F>(t1300, t198, t336, t89888, t89930, t90293, t90321, t90323, t90327, t90329, t90332, t90336, t90339, t90341, t90343, t90346, t90349, t91440, t91748);
        let t91758 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1806::<F>(t1832, t5023, t81139, t90351, t90356, t90361, t90364, t90367, t90370, t90373, t90375, t90377, t90503, t90505, t90509);
        let t91765 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1807::<F>(t6752, t198, t336, t44126, t90511, t90514, t90578, t90580, t90582, t90585, t90588, t90592, t90594, t90597, t90599, t90602);
        let t91774 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1808::<F>(t6748, t198, t336, t3801, t5023, t6752, t73252, t90629, t90631, t90634, t90636, t90640, t90644, t90855, t90857, t90860, t90863, t90867);
        let t91789 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1809::<F>(t33, t265, t502, t87990, t91754, t91758, t91765, t91774, t1469, t1587, t1711, t1837, t22671, t22783, t23436, t25032, t504, t57, t5825, t6084, t6416, t6757, t87126, t89780, dens_threshold, rho1, zeta_threshold);
        let (t91797, t91802, t91810, t91811, t91816, t91824) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1810::<F>(t30, t33, t6785, t5824, t1344, t21944, t22670, t3874, t46310, t5574, t87125, t6792, t6416, t1348, t21956, t22783, t3881, t46328, t5582, t89780, zeta_threshold);
        let (t91826, t91865, t91870, t91875, t91882) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1811::<F>(t91810, t91824, t543, t6816, t6836, t13804, t1410, t1868, t22046, t22079, t22893, t3934, t3936, t4003, t4012, t46627, t5671, t6869, t73778, t73789, t828, t85514, t85516, t85532, t85543, t85545, t85553, t85609, t85648, t85652, t9955, t9994);
    (t91789, t91797, t91802, t91811, t91816, t91826, t91865, t91870, t91875, t91882)
}
