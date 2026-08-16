//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta648 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2154;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2155;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2156;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2157;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta648<F: Float>(t214: F, t4265: F, t1880: F, t6572: F, t25055: F, t81591: F, t13049: F, t13065: F, t13072: F, t13461: F, t1492: F, t22975: F, t23150: F, t25168: F, t25170: F, t259: F, t4268: F, t6627: F, t6663: F, t82154: F, t82172: F, t82174: F, t82182: F, t866: F, t87746: F, t87748: F, t87754: F, t87755: F, t87758: F, t87765: F, t87773: F, t87777: F, t87779: F, t25217: F, t6547: F, t25060: F, t82209: F, t82211: F, t225: F, t25222: F, t23237: F, t25216: F, t1912: F, t218: F, t23281: F, t4273: F, t46508: F, t47618: F, t7517: F, t82219: F, t82221: F, t82230: F, t82236: F, t87512: F, t9593: F, t25192: F, t81651: F, t82074: F, t25220: F, t82259: F, t6552: F, t6555: F, t23270: F, t25038: F, t25191: F, t87036: F, t25054: F, t13042: F, t13463: F, t25188: F, t25200: F, t25348: F, t2713: F, t2718: F, t2720: F, t2743: F, t4300: F, t47585: F, t6632: F, t6662: F, t855: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t87782, t87792) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2154::<F>(t214, t4265, t1880, t6572, t25055, t81591, t13049, t13065, t13072, t13461, t1492, t22975, t23150, t25168, t25170, t259, t4268, t6627, t6663, t82154, t82172, t82174, t82182, t866, t87746, t87748, t87754, t87755, t87758, t87765, t87773, t87777, t87779);
        let (t87797, t87805, t87806, t87807, t87827) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2155::<F>(t25217, t6547, t25060, t82209, t82211, t225, t25222, t1880, t23237, t25216, t1912, t218, t23281, t259, t4273, t46508, t47618, t7517, t82219, t82221, t82230, t82236, t866, t87512, t9593);
        let (t87836, t87837, t87847, t87861, t87866) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2156::<F>(t25192, t81651, t82074, t225, t25220, t82259, t6552, t6555, t87782, t23270, t25038, t25191, t87036);
        let t87880 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2157::<F>(t25054, t81651, t82074, t13042, t13065, t13463, t1912, t25188, t25200, t25348, t2713, t2718, t2720, t2743, t4300, t47585, t6632, t6662, t6663, t855, t87861, t87866);
    (t87792, t87797, t87805, t87806, t87807, t87827, t87836, t87837, t87847, t87880)
}
