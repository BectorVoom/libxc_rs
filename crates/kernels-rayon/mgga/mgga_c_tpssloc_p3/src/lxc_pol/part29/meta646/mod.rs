//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta646 (260520-c91 hierarchical CSE).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2132;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2133;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2134;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2135;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2136;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2137;
use chunk6::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2138;
use chunk7::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2139;
use chunk8::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2140;
use chunk9::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2141;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta646(t232: f64, t46693: f64, t6605: f64, t815: f64, t2628: f64, t58345: f64, t2632: f64, t47262: f64, t22996: f64, t6590: f64, t25130: f64, t828: f64, t9627: f64, t81955: f64, t81957: f64, t81964: f64, t87458: f64, t87464: f64, t87466: f64, t87469: f64, t87472: f64, t87475: f64, t87478: f64, t87481: f64, t87485: f64, t87488: f64, t87491: f64, t87221: f64, t87259: f64, t87286: f64, t87324: f64, t87377: f64, t87415: f64, t87455: f64, t22986: f64, t25249: f64, t2679: f64, t6646: f64, t23110: f64, t25299: f64, t81651: f64, t23168: f64, t25313: f64, t13176: f64, t226: f64, t235: f64, t25256: f64, t25261: f64, t2617: f64, t4281: f64, t6658: f64, t81617: f64, t87150: f64, t87154: f64, t87155: f64, t87159: f64, t87166: f64, t87167: f64, t87171: f64, t87174: f64, t87177: f64, t9632: f64, t25319: f64, t2553: f64, t6552: f64, t6637: f64, t252: f64, t87230: f64, t13230: f64, t87052: f64, t25321: f64, t25284: f64, t6579: f64, t13388: f64, t1888: f64, t13385: f64, t23185: f64, t4283: f64, t81914: f64, t25300: f64, t81591: f64, t1484: f64, t81658: f64, t81633: f64, t13453: f64, t1499: f64, t23151: f64, t25281: f64, t2684: f64, t4291: f64, t81623: f64, t81630: f64, t81642: f64, t81653: f64, t25303: f64, t1509: f64, t6624: f64, t13456: f64, t13450: f64, t4292: f64, t25288: f64, t234: f64, t4265: f64, t776: f64, t25237: f64, t25307: f64, t13263: f64, t13397: f64, t2633: f64, t4182: f64, t81656: f64, t81670: f64, t81689: f64, t81691: f64, t829: f64, t10007: f64, t4282: f64, t25287: f64, t13401: f64, t22893: f64, t23164: f64, t25320: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t87495, t87498, t87502, t87507) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2132(t232, t46693, t6605, t815, t2628, t58345, t2632, t47262, t22996, t6590, t25130, t828, t9627);
        let t87509 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2133(t81955, t81957, t81964, t87458, t87464, t87466, t87469, t87472, t87475, t87478, t87481, t87485, t87488, t87491, t87495, t87498, t87502, t87507);
        let (t87512, t87517) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2134(t87221, t87259, t87286, t87324, t87377, t87415, t87455, t87509, t22986, t25249, t2679, t6646);
        let t87524 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2135(t23110, t25299, t81651, t23168, t25313, t13176, t226, t235, t25256, t25261, t2617, t4281, t6658, t81617, t87150, t87154, t87155, t87159, t87166, t87167, t87171, t87174, t87177, t87512, t87517, t9632);
        let (t87527, t87531, t87534, t87535) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2136(t25319, t2553, t6552, t6637, t252, t87230, t13230, t87052, t23168, t25321, t25284, t6579);
        let (t87536, t87538, t87541, t87545, t87547, t87554) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2137(t87535, t13388, t1888, t6646, t13385, t22996, t23185, t4283, t81914, t25300, t81591, t1484, t6552, t6637, t81658);
        let t87562 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2138(t81633, t13453, t1499, t23151, t25261, t25281, t2684, t4291, t81623, t81630, t81642, t81653, t87527, t87531, t87534, t87536, t87538, t87541, t87545, t87547, t87554);
        let (t87566, t87567, t87575, t87578, t87582, t87583) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2139(t25303, t6579, t1509, t6624, t13456, t1888, t6646, t13450, t23110, t23185, t4292, t25288, t81591);
        let t87606 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2140(t87583, t234, t4265, t6552, t6637, t776, t23110, t23185, t25237, t23168, t25307, t13263, t13397, t25261, t2633, t2679, t4182, t4281, t4291, t81656, t81670, t81689, t81691, t829, t87566, t87567, t87575, t87578, t87582);
        let (t87609, t87613, t87615, t87618) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2141(t10007, t22986, t4282, t6646, t23110, t25287, t81651, t13401, t1888, t22996, t22893, t23164, t25320);
    (t87512, t87524, t87562, t87606, t87609, t87613, t87615, t87618)
}
