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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta631<F: Float>(t87533: F, t87535: F, t87544: F, t87546: F, t87197: F, t87205: F, t87211: F, t81750: F, t84857: F, t84859: F, t87183: F, t87185: F, t87187: F, t87189: F, t87191: F, t87193: F, t87195: F, t87200: F, t87213: F, t87216: F, t87219: F, t87233: F, t87243: F, t87247: F, t87255: F, t81764: F, t81770: F, t81772: F, t81785: F, t87222: F, t87224: F, t87226: F, t87235: F, t87241: F, t87245: F, t87249: F, t87251: F, t87253: F, t87257: F, t87262: F, t87270: F, t87272: F, t81789: F, t81795: F, t81797: F, t81799: F, t81808: F, t81810: F, t81825: F, t81836: F, t84896: F, t84897: F, t87274: F, t87276: F, t87278: F, t87280: F, t87284: F, t87291: F, t87293: F, t87300: F, t87304: F, t87308: F, t81857: F, t81859: F, t81874: F, t81877: F, t81883: F, t87287: F, t87289: F, t87296: F, t87298: F, t87306: F, t87312: F, t87316: F, t87322: F, t87328: F, t87330: F, t87332: F, t87338: F, t87341: F, t87345: F, t87347: F, t87363: F, t87335: F, t87343: F, t87351: F, t87355: F, t87359: F, t87365: F, t87369: F, t87371: F, t87373: F, t87375: F, t87401: F, t87403: F, t87405: F, t87411: F, t81887: F, t81889: F, t81899: F, t81903: F, t81909: F, t81912: F, t87379: F, t87381: F, t87387: F, t87389: F, t87391: F, t87395: F, t87399: F, t87409: F, t87432: F, t87443: F, t81918: F, t81924: F, t81926: F, t81928: F, t81934: F, t81936: F, t81943: F, t84921: F, t87418: F, t87422: F, t87425: F, t87428: F, t87430: F, t87445: F, t87449: F, t87453: F, t87463: F, t87477: F, t87487: F, t81957: F, t81964: F, t84932: F, t87458: F, t87466: F, t87469: F, t87472: F, t87475: F, t87481: F, t87485: F, t87491: F, t87495: F, t87498: F, t87502: F, t87507: F, t87565: F, t226: F, t235: F, t24269: F, t26661: F, t2684: F, t4234: F, t812: F, t81623: F, t81630: F, t81633: F, t81642: F, t81653: F, t87531: F, t87538: F, t87541: F, t87554: F, t87581: F, t87583: F, t2047: F, t4233: F, t87601: F, t87603: F, t13176: F, t24270: F, t2617: F, t26608: F, t26656: F, t4166: F, t4281: F, t4291: F, t7102: F, t81656: F, t81670: F, t81691: F, t829: F, t84995: F, t87575: F, t87578: F, t87589: F, t87609: F, t9632: F, t87612: F, t87618: F, t87653: F, t13263: F, t13336: F, t13397: F, t2051: F, t2633: F, t81697: F, t81704: F, t87615: F, t87627: F, t87630: F, t87633: F, t87635: F, t87640: F, t87645: F, t87650: F, t87666: F, t87668: F, t87679: F, t13390: F, t1499: F, t24251: F, t24278: F, t26676: F, t4182: F, t81980: F, t81989: F, t82005: F, t82011: F, t82013: F, t82016: F, t85003: F, t87660: F, t87672: F, t87676: F) -> (F, F, F, F, F) {
        let (t92560, t92561, t92564, t92565, t92586) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1977::<F>(t87533, t87535, t87544, t87546, t87197, t87205, t87211, t81750, t84857, t84859, t87183, t87185, t87187, t87189, t87191, t87193, t87195, t87200, t87213, t87216, t87219);
        let t92605 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1978::<F>(t87233, t87243, t87247, t87255, t81764, t81770, t81772, t81785, t87222, t87224, t87226, t87235, t87241, t87245, t87249, t87251, t87253, t87257);
        let t92623 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1979::<F>(t87262, t87270, t87272, t81789, t81795, t81797, t81799, t81808, t81810, t81825, t81836, t84896, t84897, t87274, t87276, t87278, t87280, t87284);
        let t92642 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1980::<F>(t87291, t87293, t87300, t87304, t87308, t81857, t81859, t81874, t81877, t81883, t87287, t87289, t87296, t87298, t87306, t87312, t87316, t87322);
        let t92663 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1981::<F>(t87328, t87330, t87332, t87338, t87341, t87345, t87347, t87363, t87335, t87343, t87351, t87355, t87359, t87365, t87369, t87371, t87373, t87375);
        let t92682 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1982::<F>(t87401, t87403, t87405, t87411, t81887, t81889, t81899, t81903, t81909, t81912, t87379, t87381, t87387, t87389, t87391, t87395, t87399, t87409);
        let t92701 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1983::<F>(t87432, t87443, t81918, t81924, t81926, t81928, t81934, t81936, t81943, t84921, t87418, t87422, t87425, t87428, t87430, t87445, t87449, t87453);
        let t92719 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1984::<F>(t87463, t87477, t87487, t81957, t81964, t84932, t87458, t87466, t87469, t87472, t87475, t87481, t87485, t87491, t87495, t87498, t87502, t87507);
        let (t92722, t92732) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1985::<F>(t92586, t92605, t92623, t92642, t92663, t92682, t92701, t92719, t87565, t226, t235, t24269, t26661, t2684, t4234, t812, t81623, t81630, t81633, t81642, t81653, t87531, t87538, t87541, t87554, t92560, t92561, t92564, t92565);
        let (t92745, t92759) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1986::<F>(t87581, t87583, t2047, t4233, t87601, t87603, t13176, t24270, t2617, t26608, t26656, t4166, t4281, t4291, t7102, t81656, t81670, t81691, t829, t84995, t87575, t87578, t87589, t87609, t9632);
        let t92782 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1987::<F>(t87612, t87618, t87653, t13263, t13336, t13397, t2051, t2633, t26656, t2684, t4281, t4291, t81697, t81704, t87615, t87627, t87630, t87633, t87635, t87640, t87645, t87650);
        let t92803 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1988::<F>(t87666, t87668, t87679, t13390, t1499, t24251, t24278, t26676, t4166, t4182, t4281, t81980, t81989, t82005, t82011, t82013, t82016, t85003, t87660, t87672, t87676, t92745);
    (t92722, t92732, t92759, t92782, t92803)
}
