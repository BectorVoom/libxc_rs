//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta451 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1717;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1718;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1719;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1720;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1721;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1722;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1723;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta451(t1353: f64, t13767: f64, t2661: f64, t3889: f64, t4010: f64, t240: f64, t9991: f64, t550: f64, t9898: f64, t9994: f64, t3992: f64, t543: f64, t9890: f64, t3995: f64, t40488: f64, t3989: f64, t9944: f64, t549: f64, t72: f64, t3829: f64, t4014: f64, t9779: f64, t221: f64, t3978: f64, t3979: f64, t9628: f64, t1408: f64, t2237: f64, t2482: f64, t3981: f64, t1412: f64, t808: f64, t9736: f64, t1369: f64, t9726: f64, t1372: f64, t1410: f64, t1414: f64, t46345: f64, t46592: f64, t46596: f64, t46598: f64, t46600: f64, t46602: f64, t828: f64, t125: f64, t13999: f64, t9837: f64, t546: f64, t9801: f64, t9738: f64, t124: f64, t3938: f64, t4056: f64, t9816: f64, t9818: f64, t794: f64, t9747: f64, t9750: f64, t2699: f64, t3943: f64, t3946: f64, t40690: f64, t9775: f64, t9936: f64, t3970: f64, t9765: f64, t9923: f64, t1399: f64, t3934: f64, t3936: f64, t4012: f64, t4057: f64, t46298: f64, t5671: f64, t5673: f64, t9810: f64, t9826: f64, t9835: f64, t9840: f64, t136: f64, t9941: f64, t9400: f64, t1386: f64, t820: f64, t9948: f64, t1401: f64, t159: f64, t216: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46607, t46610, t46613, t46618) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1717(t1353, t13767, t2661, t3889, t4010, t240, t9991, t550, t9898, t9994, t3992, t543, t9890);
        let (t46620, t46622, t46627, t46628, t46633, t46641) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1718(t3995, t40488, t3989, t9944, t549, t240, t72, t3829, t4014, t9779, t221, t3978, t3979, t9628);
        let t46654 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1719(t1408, t2237, t2482, t3981, t1412, t3889, t808, t9736, t1369, t9726, t1372, t1410, t1414, t46345, t46592, t46596, t46598, t46600, t46602, t46607, t46613, t46618, t46620, t46622, t46627, t46628, t46633, t46641, t828);
        let (t46655, t46660, t46671, t46680) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1720(t125, t9898, t13999, t9837, t546, t9801, t9738, t124, t3938, t4056, t9816, t9818);
        let (t46682, t46692, t46695, t46702, t46704) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1721(t125, t9890, t794, t9747, t9750, t2699, t3943, t3946, t3995, t40690, t9775, t9936);
        let t46714 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1722(t3970, t9779, t9765, t9923, t125, t1399, t1410, t3934, t3936, t3938, t4012, t4057, t46298, t46655, t46660, t46671, t46680, t46682, t46692, t46695, t46702, t46704, t5671, t5673, t828, t9628, t9810, t9826, t9835, t9840);
        let (t46719, t46723, t46730) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1723(t136, t9941, t221, t3978, t9400, t1386, t820, t9948, t1401, t159, t216, t4010);
    (t46610, t46628, t46654, t46655, t46682, t46714, t46719, t46723, t46730)
}
