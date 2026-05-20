//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta970 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3270;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3271;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3272;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3273;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3274;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3275;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3276;
use chunk7::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3277;
use chunk8::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3278;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta970<F: Float>(t47125: F, t47127: F, t47135: F, t48324: F, t47147: F, t48335: F, t40076: F, t40079: F, t47131: F, t47138: F, t47140: F, t47142: F, t47152: F, t48327: F, t48330: F, t48332: F, t48334: F, t1353: F, t13768: F, t13902: F, t13910: F, t1392: F, t1394: F, t1412: F, t1877: F, t21969: F, t22236: F, t22249: F, t22287: F, t22809: F, t22813: F, t22944: F, t22947: F, t22950: F, t539: F, t5591: F, t5650: F, t5651: F, t6816: F, t85442: F, t9940: F, t1395: F, t1879: F, t22223: F, t22229: F, t22237: F, t22240: F, t22246: F, t225: F, t22936: F, t541: F, t543: F, t5644: F, t5652: F, t5655: F, t6832: F, t6837: F, t6840: F, t73: F, t85892: F, t85901: F, t85907: F, t85915: F, t85927: F, t85977: F, t85988: F, t221: F, t22954: F, t4018: F, t4019: F, t22893: F, t2661: F, t3992: F, t48455: F, t22858: F, t47293: F, t10001: F, t22863: F, t22914: F, t3930: F, t124: F, t1370: F, t13783: F, t1388: F, t1390: F, t1410: F, t3934: F, t46627: F, t46831: F, t46833: F, t46840: F, t46859: F, t48756: F, t5627: F, t6844: F, t74402: F, t74421: F, t800: F, t828: F, t85873: F, t85885: F, t22074: F, t47248: F, t48712: F, t48792: F, t48794: F, t48797: F, t48814: F, t48827: F, t48829: F, t48833: F, t48848: F, t48849: F, t48851: F, t48853: F, t74425: F, t74427: F, t74429: F, t74437: F, t74461: F, t74469: F, t22865: F, t9918: F, t1883: F, t6883: F, t9816: F, t9818: F, t1399: F, t22046: F, t22096: F, t3936: F, t48869: F, t48872: F, t48877: F, t48879: F, t5659: F, t5673: F, t74471: F, t74475: F, t74479: F, t74481: F, t74485: F, t74489: F, t74491: F, t74493: F, t74498: F, t85609: F, t13999: F, t22833: F, t13944: F, t21990: F, t46879: F, t46885: F, t46889: F, t46947: F, t47199: F, t48881: F, t48905: F, t48909: F, t48947: F, t48982: F, t5671: F, t74505: F, t74507: F, t74511: F, t74522: F, t74547: F, t9955: F, t547: F, t807: F, t9941: F, t1413: F, t13767: F, t1868: F, t74012: F, t13789: F, t13790: F, t22079: F, t47262: F, t49008: F, t49012: F, t49030: F, t49057: F, t73847: F, t74579: F, t74583: F, t74585: F, t74589: F, t74598: F, t74602: F, t74606: F, t85625: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t85989, t85990, t85991, t85992, t85993, t85994, t85995) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3270::<F>(t47125, t47127, t47135, t48324, t47147, t48335, t40076, t40079, t47131, t47138, t47140, t47142, t47152, t48327, t48330, t48332, t48334);
        let t86052 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3271::<F>(t1353, t13768, t13902, t13910, t1392, t1394, t1412, t1877, t21969, t22236, t22249, t22287, t22809, t22813, t22944, t22947, t22950, t539, t5591, t5650, t5651, t6816, t85442, t9940);
        let t86054 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3272::<F>(t1395, t1879, t22223, t22229, t22237, t22240, t22246, t225, t22936, t541, t543, t5644, t5652, t5655, t6832, t6837, t6840, t73, t85892, t85901, t85907, t85915, t85927, t85977, t85988, t85995, t86052);
        let (t86061, t86070, t86074, t86078) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3273::<F>(t221, t22954, t4018, t4019, t22893, t2661, t3992, t48455, t22858, t47293, t10001, t22863);
        let t86086 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3274::<F>(t22914, t3930, t124, t1353, t1370, t13783, t1388, t1390, t1410, t22813, t3934, t46627, t46831, t46833, t46840, t46859, t48756, t5627, t6844, t74402, t74421, t800, t828, t85442, t85873, t85885, t86054, t86061, t86070, t86074, t86078);
        let t86106 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3275::<F>(t22074, t47248, t48712, t48792, t48794, t48797, t48814, t48827, t48829, t48833, t48848, t48849, t48851, t48853, t5627, t74425, t74427, t74429, t74437, t74461, t74469);
        let t86136 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3276::<F>(t22865, t9918, t1883, t6883, t9816, t9818, t1399, t22046, t22074, t22096, t3934, t3936, t48869, t48872, t48877, t48879, t5659, t5673, t74471, t74475, t74479, t74481, t74485, t74489, t74491, t74493, t74498, t85609);
        let t86162 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3277::<F>(t13999, t22833, t13944, t21990, t22046, t22893, t3934, t46879, t46885, t46889, t46947, t47199, t48881, t48905, t48909, t48947, t48982, t5671, t5673, t74505, t74507, t74511, t74522, t74547, t9955);
        let t86198 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3278::<F>(t22813, t547, t807, t9941, t1413, t22809, t13767, t1868, t2661, t74012, t13789, t13790, t21990, t22079, t47262, t49008, t49012, t49030, t49057, t5671, t5673, t73847, t74579, t74583, t74585, t74589, t74598, t74602, t74606, t85625);
    (t85989, t85990, t85991, t85992, t85993, t85994, t86054, t86086, t86106, t86136, t86162, t86198)
}
