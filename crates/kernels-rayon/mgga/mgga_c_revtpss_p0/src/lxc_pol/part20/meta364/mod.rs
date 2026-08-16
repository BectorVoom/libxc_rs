//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta364 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1325;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1326;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1327;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1328;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1329;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta364(t57: f64, t202: f64, t635: f64, t10326: f64, t10457: f64, t10460: f64, t2251: f64, t2258: f64, t2382: f64, t39443: f64, t39449: f64, t39457: f64, t81: f64, zeta_threshold: f64, t39838: f64, t162: f64, t187: f64, t10428: f64, t2615: f64, t2622: f64, t9586: f64, t2514: f64, t2492: f64, t2548: f64, t2490: f64, t2595: f64, t39490: f64, t39492: f64, t39495: f64, t39498: f64, t39501: f64, t39506: f64, t39508: f64, t39510: f64, t39512: f64, t39515: f64, t177: f64, t2495: f64, t2537: f64, t2539: f64, t2554: f64, t2556: f64, t2557: f64, t2597: f64, t2598: f64, t2604: f64, t39419: f64, t39422: f64, t39483: f64, t39520: f64, t39528: f64, t39531: f64, t729: f64, t730: f64, t731: f64, t739: f64, t745: f64, t9371: f64, t9433: f64, t9446: f64, t9536: f64, t123: f64, t173: f64, t2536: f64, t2538: f64, t2549: f64, t2553: f64, t2605: f64, t39500: f64, t39747: f64, t39750: f64, t39756: f64, t39760: f64, t39773: f64, t39783: f64, t39786: f64, t39815: f64, t689: f64, t724: f64, t744: f64, t9318: f64, t9323: f64, t9480: f64, t9485: f64, t9530: f64, t9532: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t39853 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1325(t57, t202, t635, t10326, t10457, t10460, t2251, t2258, t2382, t39443, t39449, t39457, t81, zeta_threshold);
        let (t39854, t39857, t39859, t39861, t39871, t39875) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1326(t39838, t39853, t162, t187, t10428, t2615, t2622, t9586, t2514, t2492);
        let (t39886, t39894, t39909) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1327(t2548, t2490, t2595, t39490, t39492, t39495, t39498, t39501, t39506, t39508, t39510, t39512, t39515);
        let t39913 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1328(t177, t2495, t2514, t2537, t2539, t2548, t2554, t2556, t2557, t2597, t2598, t2604, t39419, t39422, t39483, t39520, t39528, t39531, t39871, t39875, t39886, t39894, t39909, t729, t730, t731, t739, t745, t9371, t9433, t9446, t9536);
        let t39957 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1329(t123, t173, t2514, t2536, t2538, t2548, t2549, t2553, t2556, t2597, t2604, t2605, t39490, t39492, t39495, t39498, t39500, t39501, t39506, t39508, t39510, t39512, t39515, t39747, t39750, t39756, t39760, t39773, t39783, t39786, t39815, t689, t724, t729, t730, t744, t9318, t9323, t9480, t9485, t9530, t9532);
    (t39854, t39857, t39859, t39861, t39871, t39875, t39886, t39894, t39909, t39913, t39957)
}
