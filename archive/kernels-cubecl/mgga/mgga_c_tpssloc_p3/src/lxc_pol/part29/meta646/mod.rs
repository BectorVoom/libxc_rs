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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta646<F: Float>(t232: F, t46693: F, t6605: F, t815: F, t2628: F, t58345: F, t2632: F, t47262: F, t22996: F, t6590: F, t25130: F, t828: F, t9627: F, t81955: F, t81957: F, t81964: F, t87458: F, t87464: F, t87466: F, t87469: F, t87472: F, t87475: F, t87478: F, t87481: F, t87485: F, t87488: F, t87491: F, t87221: F, t87259: F, t87286: F, t87324: F, t87377: F, t87415: F, t87455: F, t22986: F, t25249: F, t2679: F, t6646: F, t23110: F, t25299: F, t81651: F, t23168: F, t25313: F, t13176: F, t226: F, t235: F, t25256: F, t25261: F, t2617: F, t4281: F, t6658: F, t81617: F, t87150: F, t87154: F, t87155: F, t87159: F, t87166: F, t87167: F, t87171: F, t87174: F, t87177: F, t9632: F, t25319: F, t2553: F, t6552: F, t6637: F, t252: F, t87230: F, t13230: F, t87052: F, t25321: F, t25284: F, t6579: F, t13388: F, t1888: F, t13385: F, t23185: F, t4283: F, t81914: F, t25300: F, t81591: F, t1484: F, t81658: F, t81633: F, t13453: F, t1499: F, t23151: F, t25281: F, t2684: F, t4291: F, t81623: F, t81630: F, t81642: F, t81653: F, t25303: F, t1509: F, t6624: F, t13456: F, t13450: F, t4292: F, t25288: F, t234: F, t4265: F, t776: F, t25237: F, t25307: F, t13263: F, t13397: F, t2633: F, t4182: F, t81656: F, t81670: F, t81689: F, t81691: F, t829: F, t10007: F, t4282: F, t25287: F, t13401: F, t22893: F, t23164: F, t25320: F) -> (F, F, F, F, F, F, F, F) {
        let (t87495, t87498, t87502, t87507) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2132::<F>(t232, t46693, t6605, t815, t2628, t58345, t2632, t47262, t22996, t6590, t25130, t828, t9627);
        let t87509 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2133::<F>(t81955, t81957, t81964, t87458, t87464, t87466, t87469, t87472, t87475, t87478, t87481, t87485, t87488, t87491, t87495, t87498, t87502, t87507);
        let (t87512, t87517) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2134::<F>(t87221, t87259, t87286, t87324, t87377, t87415, t87455, t87509, t22986, t25249, t2679, t6646);
        let t87524 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2135::<F>(t23110, t25299, t81651, t23168, t25313, t13176, t226, t235, t25256, t25261, t2617, t4281, t6658, t81617, t87150, t87154, t87155, t87159, t87166, t87167, t87171, t87174, t87177, t87512, t87517, t9632);
        let (t87527, t87531, t87534, t87535) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2136::<F>(t25319, t2553, t6552, t6637, t252, t87230, t13230, t87052, t23168, t25321, t25284, t6579);
        let (t87536, t87538, t87541, t87545, t87547, t87554) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2137::<F>(t87535, t13388, t1888, t6646, t13385, t22996, t23185, t4283, t81914, t25300, t81591, t1484, t6552, t6637, t81658);
        let t87562 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2138::<F>(t81633, t13453, t1499, t23151, t25261, t25281, t2684, t4291, t81623, t81630, t81642, t81653, t87527, t87531, t87534, t87536, t87538, t87541, t87545, t87547, t87554);
        let (t87566, t87567, t87575, t87578, t87582, t87583) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2139::<F>(t25303, t6579, t1509, t6624, t13456, t1888, t6646, t13450, t23110, t23185, t4292, t25288, t81591);
        let t87606 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2140::<F>(t87583, t234, t4265, t6552, t6637, t776, t23110, t23185, t25237, t23168, t25307, t13263, t13397, t25261, t2633, t2679, t4182, t4281, t4291, t81656, t81670, t81689, t81691, t829, t87566, t87567, t87575, t87578, t87582);
        let (t87609, t87613, t87615, t87618) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2141::<F>(t10007, t22986, t4282, t6646, t23110, t25287, t81651, t13401, t1888, t22996, t22893, t23164, t25320);
    (t87512, t87524, t87562, t87606, t87609, t87613, t87615, t87618)
}
