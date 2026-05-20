//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta451 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1717;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1718;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1719;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1720;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1721;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1722;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1723;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta451<F: Float>(t1353: F, t13767: F, t2661: F, t3889: F, t4010: F, t240: F, t9991: F, t550: F, t9898: F, t9994: F, t3992: F, t543: F, t9890: F, t3995: F, t40488: F, t3989: F, t9944: F, t549: F, t72: F, t3829: F, t4014: F, t9779: F, t221: F, t3978: F, t3979: F, t9628: F, t1408: F, t2237: F, t2482: F, t3981: F, t1412: F, t808: F, t9736: F, t1369: F, t9726: F, t1372: F, t1410: F, t1414: F, t46345: F, t46592: F, t46596: F, t46598: F, t46600: F, t46602: F, t828: F, t125: F, t13999: F, t9837: F, t546: F, t9801: F, t9738: F, t124: F, t3938: F, t4056: F, t9816: F, t9818: F, t794: F, t9747: F, t9750: F, t2699: F, t3943: F, t3946: F, t40690: F, t9775: F, t9936: F, t3970: F, t9765: F, t9923: F, t1399: F, t3934: F, t3936: F, t4012: F, t4057: F, t46298: F, t5671: F, t5673: F, t9810: F, t9826: F, t9835: F, t9840: F, t136: F, t9941: F, t9400: F, t1386: F, t820: F, t9948: F, t1401: F, t159: F, t216: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t46607, t46610, t46613, t46618) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1717::<F>(t1353, t13767, t2661, t3889, t4010, t240, t9991, t550, t9898, t9994, t3992, t543, t9890);
        let (t46620, t46622, t46627, t46628, t46633, t46641) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1718::<F>(t3995, t40488, t3989, t9944, t549, t240, t72, t3829, t4014, t9779, t221, t3978, t3979, t9628);
        let t46654 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1719::<F>(t1408, t2237, t2482, t3981, t1412, t3889, t808, t9736, t1369, t9726, t1372, t1410, t1414, t46345, t46592, t46596, t46598, t46600, t46602, t46607, t46613, t46618, t46620, t46622, t46627, t46628, t46633, t46641, t828);
        let (t46655, t46660, t46671, t46680) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1720::<F>(t125, t9898, t13999, t9837, t546, t9801, t9738, t124, t3938, t4056, t9816, t9818);
        let (t46682, t46692, t46695, t46702, t46704) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1721::<F>(t125, t9890, t794, t9747, t9750, t2699, t3943, t3946, t3995, t40690, t9775, t9936);
        let t46714 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1722::<F>(t3970, t9779, t9765, t9923, t125, t1399, t1410, t3934, t3936, t3938, t4012, t4057, t46298, t46655, t46660, t46671, t46680, t46682, t46692, t46695, t46702, t46704, t5671, t5673, t828, t9628, t9810, t9826, t9835, t9840);
        let (t46719, t46723, t46730) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1723::<F>(t136, t9941, t221, t3978, t9400, t1386, t820, t9948, t1401, t159, t216, t4010);
    (t46610, t46628, t46654, t46655, t46682, t46714, t46719, t46723, t46730)
}
