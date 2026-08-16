//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta711 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2225;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2226;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2227;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2228;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2229;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2230;
use chunk6::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2231;
use chunk7::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2232;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta711<F: Float>(t28342: F, t81979: F, t17022: F, t1880: F, t1894: F, t214: F, t252: F, t5527: F, t25038: F, t6646: F, t829: F, t28333: F, t6562: F, t794: F, t22893: F, t23164: F, t28345: F, t23153: F, t5544: F, t6552: F, t6637: F, t16662: F, t6638: F, t28329: F, t16927: F, t87052: F, t87529: F, t23185: F, t28426: F, t81914: F, t25248: F, t776: F, t87642: F, t81575: F, t87067: F, t87078: F, t92492: F, t98325: F, t98328: F, t28334: F, t6547: F, t28322: F, t6579: F, t16762: F, t1888: F, t16828: F, t1484: F, t1519: F, t232: F, t58262: F, t23110: F, t28422: F, t16817: F, t82018: F, t16825: F, t22996: F, t1510: F, t16673: F, t16753: F, t2617: F, t28351: F, t28409: F, t28411: F, t6657: F, t6658: F, t812: F, t87101: F, t87135: F, t92497: F, t23168: F, t28346: F, t28338: F, t81591: F, t22986: F, t16759: F, t17030: F, t2647: F, t17046: F, t87130: F, t25249: F, t4234: F, t28337: F, t81651: F, t13176: F, t1499: F, t22992: F, t25295: F, t5617: F, t7533: F, t81595: F, t81599: F, t81600: F, t81602: F, t92513: F, t87111: F, t16820: F, t17031: F, t16815: F, t9627: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t98330, t98334, t98336, t98339, t98342) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2225::<F>(t28342, t81979, t17022, t1880, t1894, t214, t252, t5527, t25038, t6646, t829, t28333, t6562, t794);
        let (t98345, t98349, t98353, t98356, t98359) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2226::<F>(t22893, t23164, t28345, t23153, t5544, t6552, t6637, t16662, t6638, t28329, t16927, t87052, t87529);
        let t98370 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2227::<F>(t23185, t28426, t81914, t25248, t776, t87642, t98336, t81575, t87067, t87078, t92492, t98325, t98328, t98330, t98334, t98339, t98342, t98345, t98349, t98353, t98356, t98359);
        let (t98374, t98380, t98384, t98387, t98389, t98392) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2228::<F>(t28334, t6547, t28322, t6579, t16762, t1888, t6646, t16828, t1484, t1519, t25038, t25248, t776);
        let t98409 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2229::<F>(t1888, t232, t58262, t6646, t23110, t23185, t28422, t16817, t82018, t16825, t22996, t1510, t16673, t16753, t2617, t28351, t28409, t28411, t6657, t6658, t812, t87101, t87135, t92497, t98374, t98380, t98384, t98387, t98392);
        let (t98416, t98420, t98422, t98425, t98428, t98432) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2230::<F>(t23168, t28346, t28338, t81591, t252, t5544, t22986, t6646, t829, t16759, t1888, t17030, t2647);
        let t98450 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2231::<F>(t17046, t1888, t6646, t1510, t22986, t87130, t25249, t4234, t23110, t28337, t81651, t13176, t1499, t22992, t25295, t5617, t7533, t812, t81595, t81599, t81600, t81602, t92513, t98416, t98420, t98425, t98428, t98432);
        let (t98461, t98464, t98467, t98471, t98475) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2232::<F>(t1510, t22986, t6646, t87111, t16820, t1888, t22996, t17031, t829, t98389, t16815, t9627);
    (t98370, t98409, t98422, t98450, t98461, t98464, t98467, t98471, t98475)
}
