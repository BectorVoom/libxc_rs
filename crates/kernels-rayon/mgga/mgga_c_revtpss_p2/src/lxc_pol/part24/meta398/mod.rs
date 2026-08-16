//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta398 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1325;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1326;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1327;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1328;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1329;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1330;
use chunk6::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1331;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta398(t177: f64, t2495: f64, t2514: f64, t2537: f64, t2539: f64, t2548: f64, t2554: f64, t2556: f64, t2557: f64, t2597: f64, t2598: f64, t2604: f64, t39419: f64, t39422: f64, t39483: f64, t39520: f64, t39528: f64, t39531: f64, t39871: f64, t39875: f64, t39886: f64, t39894: f64, t39909: f64, t729: f64, t730: f64, t731: f64, t739: f64, t745: f64, t9371: f64, t9433: f64, t9446: f64, t9536: f64, t123: f64, t173: f64, t2536: f64, t2538: f64, t2549: f64, t2553: f64, t2605: f64, t39490: f64, t39492: f64, t39495: f64, t39498: f64, t39500: f64, t39501: f64, t39506: f64, t39508: f64, t39510: f64, t39512: f64, t39515: f64, t39747: f64, t39750: f64, t39756: f64, t39760: f64, t39773: f64, t39783: f64, t39786: f64, t39815: f64, t689: f64, t724: f64, t744: f64, t9318: f64, t9323: f64, t9480: f64, t9485: f64, t9530: f64, t9532: f64, t2490: f64, t2494: f64, t268: f64, t675: f64, t9310: f64, t9314: f64, t164: f64, t186: f64, t215: f64, t2492: f64, t2535: f64, t2552: f64, t2591: f64, t39791: f64, t39795: f64, t39799: f64, t39807: f64, t39813: f64, t9454: f64, t2555: f64, t2564: f64, t2577: f64, t700: f64, t2576: f64, t2581: f64, t2584: f64, t2582: f64, t9305: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t39913 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1325(t177, t2495, t2514, t2537, t2539, t2548, t2554, t2556, t2557, t2597, t2598, t2604, t39419, t39422, t39483, t39520, t39528, t39531, t39871, t39875, t39886, t39894, t39909, t729, t730, t731, t739, t745, t9371, t9433, t9446, t9536);
        let t39957 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1326(t123, t173, t2514, t2536, t2538, t2548, t2549, t2553, t2556, t2597, t2604, t2605, t39490, t39492, t39495, t39498, t39500, t39501, t39506, t39508, t39510, t39512, t39515, t39747, t39750, t39756, t39760, t39773, t39783, t39786, t39815, t689, t724, t729, t730, t744, t9318, t9323, t9480, t9485, t9530, t9532);
        let (t39960, t39963, t39967, t39989) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1327(t2490, t2494, t2538, t268, t675, t9310, t9314);
        let t40007 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1328(t123, t164, t177, t186, t215, t2492, t2514, t2535, t2549, t2552, t2553, t2554, t2556, t2557, t2591, t268, t39500, t39791, t39795, t39799, t39807, t39813, t39875, t39886, t39960, t39963, t39967, t39989, t730, t745, t9371, t9433, t9454, t9480, t9485, t9530, t9532, t9536);
        let (t40056, t40059, t40067) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1329(t2552, t2555, t2564, t2577, t689, t700);
        let t40072 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1330(t2576, t2581, t2584, t689, t700);
        let t40076 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1331(t2582, t2584, t700, t9305);
    (t39913, t39957, t39960, t39963, t39967, t39989, t40007, t40056, t40059, t40067, t40072, t40076)
}
