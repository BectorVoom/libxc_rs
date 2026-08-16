//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta631 (260520-c91 hierarchical CSE).
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
mod chunk9;
mod chunk10;
mod chunk11;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1977;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1978;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1979;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1980;
use chunk4::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1981;
use chunk5::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1982;
use chunk6::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1983;
use chunk7::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1984;
use chunk8::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1985;
use chunk9::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1986;
use chunk10::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1987;
use chunk11::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1988;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta631(t87533: f64, t87535: f64, t87544: f64, t87546: f64, t87197: f64, t87205: f64, t87211: f64, t81750: f64, t84857: f64, t84859: f64, t87183: f64, t87185: f64, t87187: f64, t87189: f64, t87191: f64, t87193: f64, t87195: f64, t87200: f64, t87213: f64, t87216: f64, t87219: f64, t87233: f64, t87243: f64, t87247: f64, t87255: f64, t81764: f64, t81770: f64, t81772: f64, t81785: f64, t87222: f64, t87224: f64, t87226: f64, t87235: f64, t87241: f64, t87245: f64, t87249: f64, t87251: f64, t87253: f64, t87257: f64, t87262: f64, t87270: f64, t87272: f64, t81789: f64, t81795: f64, t81797: f64, t81799: f64, t81808: f64, t81810: f64, t81825: f64, t81836: f64, t84896: f64, t84897: f64, t87274: f64, t87276: f64, t87278: f64, t87280: f64, t87284: f64, t87291: f64, t87293: f64, t87300: f64, t87304: f64, t87308: f64, t81857: f64, t81859: f64, t81874: f64, t81877: f64, t81883: f64, t87287: f64, t87289: f64, t87296: f64, t87298: f64, t87306: f64, t87312: f64, t87316: f64, t87322: f64, t87328: f64, t87330: f64, t87332: f64, t87338: f64, t87341: f64, t87345: f64, t87347: f64, t87363: f64, t87335: f64, t87343: f64, t87351: f64, t87355: f64, t87359: f64, t87365: f64, t87369: f64, t87371: f64, t87373: f64, t87375: f64, t87401: f64, t87403: f64, t87405: f64, t87411: f64, t81887: f64, t81889: f64, t81899: f64, t81903: f64, t81909: f64, t81912: f64, t87379: f64, t87381: f64, t87387: f64, t87389: f64, t87391: f64, t87395: f64, t87399: f64, t87409: f64, t87432: f64, t87443: f64, t81918: f64, t81924: f64, t81926: f64, t81928: f64, t81934: f64, t81936: f64, t81943: f64, t84921: f64, t87418: f64, t87422: f64, t87425: f64, t87428: f64, t87430: f64, t87445: f64, t87449: f64, t87453: f64, t87463: f64, t87477: f64, t87487: f64, t81957: f64, t81964: f64, t84932: f64, t87458: f64, t87466: f64, t87469: f64, t87472: f64, t87475: f64, t87481: f64, t87485: f64, t87491: f64, t87495: f64, t87498: f64, t87502: f64, t87507: f64, t87565: f64, t226: f64, t235: f64, t24269: f64, t26661: f64, t2684: f64, t4234: f64, t812: f64, t81623: f64, t81630: f64, t81633: f64, t81642: f64, t81653: f64, t87531: f64, t87538: f64, t87541: f64, t87554: f64, t87581: f64, t87583: f64, t2047: f64, t4233: f64, t87601: f64, t87603: f64, t13176: f64, t24270: f64, t2617: f64, t26608: f64, t26656: f64, t4166: f64, t4281: f64, t4291: f64, t7102: f64, t81656: f64, t81670: f64, t81691: f64, t829: f64, t84995: f64, t87575: f64, t87578: f64, t87589: f64, t87609: f64, t9632: f64, t87612: f64, t87618: f64, t87653: f64, t13263: f64, t13336: f64, t13397: f64, t2051: f64, t2633: f64, t81697: f64, t81704: f64, t87615: f64, t87627: f64, t87630: f64, t87633: f64, t87635: f64, t87640: f64, t87645: f64, t87650: f64, t87666: f64, t87668: f64, t87679: f64, t13390: f64, t1499: f64, t24251: f64, t24278: f64, t26676: f64, t4182: f64, t81980: f64, t81989: f64, t82005: f64, t82011: f64, t82013: f64, t82016: f64, t85003: f64, t87660: f64, t87672: f64, t87676: f64) -> (f64, f64, f64, f64, f64) {
        let (t92560, t92561, t92564, t92565, t92586) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1977(t87533, t87535, t87544, t87546, t87197, t87205, t87211, t81750, t84857, t84859, t87183, t87185, t87187, t87189, t87191, t87193, t87195, t87200, t87213, t87216, t87219);
        let t92605 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1978(t87233, t87243, t87247, t87255, t81764, t81770, t81772, t81785, t87222, t87224, t87226, t87235, t87241, t87245, t87249, t87251, t87253, t87257);
        let t92623 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1979(t87262, t87270, t87272, t81789, t81795, t81797, t81799, t81808, t81810, t81825, t81836, t84896, t84897, t87274, t87276, t87278, t87280, t87284);
        let t92642 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1980(t87291, t87293, t87300, t87304, t87308, t81857, t81859, t81874, t81877, t81883, t87287, t87289, t87296, t87298, t87306, t87312, t87316, t87322);
        let t92663 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1981(t87328, t87330, t87332, t87338, t87341, t87345, t87347, t87363, t87335, t87343, t87351, t87355, t87359, t87365, t87369, t87371, t87373, t87375);
        let t92682 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1982(t87401, t87403, t87405, t87411, t81887, t81889, t81899, t81903, t81909, t81912, t87379, t87381, t87387, t87389, t87391, t87395, t87399, t87409);
        let t92701 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1983(t87432, t87443, t81918, t81924, t81926, t81928, t81934, t81936, t81943, t84921, t87418, t87422, t87425, t87428, t87430, t87445, t87449, t87453);
        let t92719 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1984(t87463, t87477, t87487, t81957, t81964, t84932, t87458, t87466, t87469, t87472, t87475, t87481, t87485, t87491, t87495, t87498, t87502, t87507);
        let (t92722, t92732) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1985(t92586, t92605, t92623, t92642, t92663, t92682, t92701, t92719, t87565, t226, t235, t24269, t26661, t2684, t4234, t812, t81623, t81630, t81633, t81642, t81653, t87531, t87538, t87541, t87554, t92560, t92561, t92564, t92565);
        let (t92745, t92759) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1986(t87581, t87583, t2047, t4233, t87601, t87603, t13176, t24270, t2617, t26608, t26656, t4166, t4281, t4291, t7102, t81656, t81670, t81691, t829, t84995, t87575, t87578, t87589, t87609, t9632);
        let t92782 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1987(t87612, t87618, t87653, t13263, t13336, t13397, t2051, t2633, t26656, t2684, t4281, t4291, t81697, t81704, t87615, t87627, t87630, t87633, t87635, t87640, t87645, t87650);
        let t92803 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1988(t87666, t87668, t87679, t13390, t1499, t24251, t24278, t26676, t4166, t4182, t4281, t81980, t81989, t82005, t82011, t82013, t82016, t85003, t87660, t87672, t87676, t92745);
    (t92722, t92732, t92759, t92782, t92803)
}
