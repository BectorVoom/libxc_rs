//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta648 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2154;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2155;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2156;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2157;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta648(t214: f64, t4265: f64, t1880: f64, t6572: f64, t25055: f64, t81591: f64, t13049: f64, t13065: f64, t13072: f64, t13461: f64, t1492: f64, t22975: f64, t23150: f64, t25168: f64, t25170: f64, t259: f64, t4268: f64, t6627: f64, t6663: f64, t82154: f64, t82172: f64, t82174: f64, t82182: f64, t866: f64, t87746: f64, t87748: f64, t87754: f64, t87755: f64, t87758: f64, t87765: f64, t87773: f64, t87777: f64, t87779: f64, t25217: f64, t6547: f64, t25060: f64, t82209: f64, t82211: f64, t225: f64, t25222: f64, t23237: f64, t25216: f64, t1912: f64, t218: f64, t23281: f64, t4273: f64, t46508: f64, t47618: f64, t7517: f64, t82219: f64, t82221: f64, t82230: f64, t82236: f64, t87512: f64, t9593: f64, t25192: f64, t81651: f64, t82074: f64, t25220: f64, t82259: f64, t6552: f64, t6555: f64, t23270: f64, t25038: f64, t25191: f64, t87036: f64, t25054: f64, t13042: f64, t13463: f64, t25188: f64, t25200: f64, t25348: f64, t2713: f64, t2718: f64, t2720: f64, t2743: f64, t4300: f64, t47585: f64, t6632: f64, t6662: f64, t855: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t87782, t87792) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2154(t214, t4265, t1880, t6572, t25055, t81591, t13049, t13065, t13072, t13461, t1492, t22975, t23150, t25168, t25170, t259, t4268, t6627, t6663, t82154, t82172, t82174, t82182, t866, t87746, t87748, t87754, t87755, t87758, t87765, t87773, t87777, t87779);
        let (t87797, t87805, t87806, t87807, t87827) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2155(t25217, t6547, t25060, t82209, t82211, t225, t25222, t1880, t23237, t25216, t1912, t218, t23281, t259, t4273, t46508, t47618, t7517, t82219, t82221, t82230, t82236, t866, t87512, t9593);
        let (t87836, t87837, t87847, t87861, t87866) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2156(t25192, t81651, t82074, t225, t25220, t82259, t6552, t6555, t87782, t23270, t25038, t25191, t87036);
        let t87880 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2157(t25054, t81651, t82074, t13042, t13065, t13463, t1912, t25188, t25200, t25348, t2713, t2718, t2720, t2743, t4300, t47585, t6632, t6662, t6663, t855, t87861, t87866);
    (t87792, t87797, t87805, t87806, t87807, t87827, t87836, t87837, t87847, t87880)
}
