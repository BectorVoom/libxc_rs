//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta364 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1325;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1326;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1327;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1328;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1329;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta364<F: Float>(t57: F, t202: F, t635: F, t10326: F, t10457: F, t10460: F, t2251: F, t2258: F, t2382: F, t39443: F, t39449: F, t39457: F, t81: F, zeta_threshold: F, t39838: F, t162: F, t187: F, t10428: F, t2615: F, t2622: F, t9586: F, t2514: F, t2492: F, t2548: F, t2490: F, t2595: F, t39490: F, t39492: F, t39495: F, t39498: F, t39501: F, t39506: F, t39508: F, t39510: F, t39512: F, t39515: F, t177: F, t2495: F, t2537: F, t2539: F, t2554: F, t2556: F, t2557: F, t2597: F, t2598: F, t2604: F, t39419: F, t39422: F, t39483: F, t39520: F, t39528: F, t39531: F, t729: F, t730: F, t731: F, t739: F, t745: F, t9371: F, t9433: F, t9446: F, t9536: F, t123: F, t173: F, t2536: F, t2538: F, t2549: F, t2553: F, t2605: F, t39500: F, t39747: F, t39750: F, t39756: F, t39760: F, t39773: F, t39783: F, t39786: F, t39815: F, t689: F, t724: F, t744: F, t9318: F, t9323: F, t9480: F, t9485: F, t9530: F, t9532: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t39853 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1325::<F>(t57, t202, t635, t10326, t10457, t10460, t2251, t2258, t2382, t39443, t39449, t39457, t81, zeta_threshold);
        let (t39854, t39857, t39859, t39861, t39871, t39875) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1326::<F>(t39838, t39853, t162, t187, t10428, t2615, t2622, t9586, t2514, t2492);
        let (t39886, t39894, t39909) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1327::<F>(t2548, t2490, t2595, t39490, t39492, t39495, t39498, t39501, t39506, t39508, t39510, t39512, t39515);
        let t39913 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1328::<F>(t177, t2495, t2514, t2537, t2539, t2548, t2554, t2556, t2557, t2597, t2598, t2604, t39419, t39422, t39483, t39520, t39528, t39531, t39871, t39875, t39886, t39894, t39909, t729, t730, t731, t739, t745, t9371, t9433, t9446, t9536);
        let t39957 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1329::<F>(t123, t173, t2514, t2536, t2538, t2548, t2549, t2553, t2556, t2597, t2604, t2605, t39490, t39492, t39495, t39498, t39500, t39501, t39506, t39508, t39510, t39512, t39515, t39747, t39750, t39756, t39760, t39773, t39783, t39786, t39815, t689, t724, t729, t730, t744, t9318, t9323, t9480, t9485, t9530, t9532);
    (t39854, t39857, t39859, t39861, t39871, t39875, t39886, t39894, t39909, t39913, t39957)
}
