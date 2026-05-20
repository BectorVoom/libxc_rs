//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta642 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2357;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2358;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2359;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2360;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2361;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta642<F: Float>(t2622: F, t9586: F, t2514: F, t2492: F, t2548: F, t2490: F, t2595: F, t39490: F, t39492: F, t39495: F, t39498: F, t39501: F, t39506: F, t39508: F, t39510: F, t39512: F, t39515: F, t177: F, t2495: F, t2537: F, t2539: F, t2554: F, t2556: F, t2557: F, t2597: F, t2598: F, t2604: F, t39419: F, t39422: F, t39483: F, t39520: F, t39528: F, t39531: F, t729: F, t730: F, t731: F, t739: F, t745: F, t9371: F, t9433: F, t9446: F, t9536: F, t123: F, t173: F, t2536: F, t2538: F, t2549: F, t2553: F, t2605: F, t39500: F, t39747: F, t39750: F, t39756: F, t39760: F, t39773: F, t39783: F, t39786: F, t39815: F, t689: F, t724: F, t744: F, t9318: F, t9323: F, t9480: F, t9485: F, t9530: F, t9532: F, t2494: F, t268: F, t675: F, t9310: F, t9314: F, t164: F, t186: F, t215: F, t2535: F, t2552: F, t2591: F, t39791: F, t39795: F, t39799: F, t39807: F, t39813: F, t9454: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t39860, t39871, t39875, t39886, t39894, t39909) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2357::<F>(t2622, t9586, t2514, t2492, t2548, t2490, t2595, t39490, t39492, t39495, t39498, t39501, t39506, t39508, t39510, t39512, t39515);
        let t39913 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2358::<F>(t177, t2495, t2514, t2537, t2539, t2548, t2554, t2556, t2557, t2597, t2598, t2604, t39419, t39422, t39483, t39520, t39528, t39531, t39871, t39875, t39886, t39894, t39909, t729, t730, t731, t739, t745, t9371, t9433, t9446, t9536);
        let t39957 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2359::<F>(t123, t173, t2514, t2536, t2538, t2548, t2549, t2553, t2556, t2597, t2604, t2605, t39490, t39492, t39495, t39498, t39500, t39501, t39506, t39508, t39510, t39512, t39515, t39747, t39750, t39756, t39760, t39773, t39783, t39786, t39815, t689, t724, t729, t730, t744, t9318, t9323, t9480, t9485, t9530, t9532);
        let (t39960, t39963, t39967, t39989) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2360::<F>(t2490, t2494, t2538, t268, t675, t9310, t9314);
        let t40007 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2361::<F>(t123, t164, t177, t186, t215, t2492, t2514, t2535, t2549, t2552, t2553, t2554, t2556, t2557, t2591, t268, t39500, t39791, t39795, t39799, t39807, t39813, t39875, t39886, t39960, t39963, t39967, t39989, t730, t745, t9371, t9433, t9454, t9480, t9485, t9530, t9532, t9536);
    (t39860, t39871, t39875, t39894, t39909, t39913, t39957, t39960, t39963, t39967, t39989, t40007)
}
