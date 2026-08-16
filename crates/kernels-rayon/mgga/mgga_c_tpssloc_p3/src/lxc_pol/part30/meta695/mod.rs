//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta695 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2219;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2220;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2221;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2222;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2223;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2224;
use chunk6::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2225;
use chunk7::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2226;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta695(t28342: f64, t81979: f64, t17022: f64, t1880: f64, t1894: f64, t214: f64, t252: f64, t5527: f64, t25038: f64, t6646: f64, t829: f64, t28333: f64, t6562: f64, t794: f64, t22893: f64, t23164: f64, t28345: f64, t23153: f64, t5544: f64, t6552: f64, t6637: f64, t16662: f64, t6638: f64, t28329: f64, t16927: f64, t87052: f64, t87529: f64, t23185: f64, t28426: f64, t81914: f64, t25248: f64, t776: f64, t87642: f64, t81575: f64, t87067: f64, t87078: f64, t92492: f64, t98325: f64, t98328: f64, t28334: f64, t6547: f64, t28322: f64, t6579: f64, t16762: f64, t1888: f64, t16828: f64, t1484: f64, t1519: f64, t232: f64, t58262: f64, t23110: f64, t28422: f64, t16817: f64, t82018: f64, t16825: f64, t22996: f64, t1510: f64, t16673: f64, t16753: f64, t2617: f64, t28351: f64, t28409: f64, t28411: f64, t6657: f64, t6658: f64, t812: f64, t87101: f64, t87135: f64, t92497: f64, t23168: f64, t28346: f64, t28338: f64, t81591: f64, t22986: f64, t16759: f64, t17030: f64, t2647: f64, t17046: f64, t87130: f64, t25249: f64, t4234: f64, t28337: f64, t81651: f64, t13176: f64, t1499: f64, t22992: f64, t25295: f64, t5617: f64, t7533: f64, t81595: f64, t81599: f64, t81600: f64, t81602: f64, t92513: f64, t87111: f64, t16820: f64, t17031: f64, t16815: f64, t9627: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t98330, t98334, t98336, t98339, t98342) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2219(t28342, t81979, t17022, t1880, t1894, t214, t252, t5527, t25038, t6646, t829, t28333, t6562, t794);
        let (t98345, t98349, t98353, t98356, t98359) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2220(t22893, t23164, t28345, t23153, t5544, t6552, t6637, t16662, t6638, t28329, t16927, t87052, t87529);
        let t98370 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2221(t23185, t28426, t81914, t25248, t776, t87642, t98336, t81575, t87067, t87078, t92492, t98325, t98328, t98330, t98334, t98339, t98342, t98345, t98349, t98353, t98356, t98359);
        let (t98374, t98380, t98384, t98387, t98389, t98392) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2222(t28334, t6547, t28322, t6579, t16762, t1888, t6646, t16828, t1484, t1519, t25038, t25248, t776);
        let t98409 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2223(t1888, t232, t58262, t6646, t23110, t23185, t28422, t16817, t82018, t16825, t22996, t1510, t16673, t16753, t2617, t28351, t28409, t28411, t6657, t6658, t812, t87101, t87135, t92497, t98374, t98380, t98384, t98387, t98392);
        let (t98416, t98420, t98422, t98425, t98428, t98432) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2224(t23168, t28346, t28338, t81591, t252, t5544, t22986, t6646, t829, t16759, t1888, t17030, t2647);
        let t98450 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2225(t17046, t1888, t6646, t1510, t22986, t87130, t25249, t4234, t23110, t28337, t81651, t13176, t1499, t22992, t25295, t5617, t7533, t812, t81595, t81599, t81600, t81602, t92513, t98416, t98420, t98425, t98428, t98432);
        let (t98461, t98464, t98467, t98471, t98475) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2226(t1510, t22986, t6646, t87111, t16820, t1888, t22996, t17031, t829, t98389, t16815, t9627);
    (t98370, t98409, t98422, t98450, t98461, t98464, t98467, t98471, t98475)
}
