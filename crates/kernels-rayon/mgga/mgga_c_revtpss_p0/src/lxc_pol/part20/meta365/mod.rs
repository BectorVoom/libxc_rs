//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta365 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1330;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1331;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1332;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1333;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1334;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1335;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1336;
use chunk7::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1337;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta365(t2490: f64, t2494: f64, t2538: f64, t268: f64, t675: f64, t9310: f64, t9314: f64, t123: f64, t164: f64, t177: f64, t186: f64, t215: f64, t2492: f64, t2514: f64, t2535: f64, t2549: f64, t2552: f64, t2553: f64, t2554: f64, t2556: f64, t2557: f64, t2591: f64, t39500: f64, t39791: f64, t39795: f64, t39799: f64, t39807: f64, t39813: f64, t39875: f64, t39886: f64, t730: f64, t745: f64, t9371: f64, t9433: f64, t9454: f64, t9480: f64, t9485: f64, t9530: f64, t9532: f64, t9536: f64, t2555: f64, t2564: f64, t2577: f64, t689: f64, t700: f64, t2576: f64, t2581: f64, t2584: f64, t2582: f64, t9305: f64, t147: f64, t2491: f64, t2531: f64, t2536: f64, t2539: f64, t2596: f64, t2598: f64, t2601: f64, t2605: f64, t723: f64, t731: f64, t738: f64, t746: f64, t793: f64, t9367: f64, t9417: f64, t9432: f64, t9435: f64, t9447: f64, t9461: f64, t9469: f64, t9476: f64, t9481: f64, t9488: f64, t9525: f64, t9529: f64, t9533: f64, t9537: f64, t39913: f64, t39957: f64, t158: f64, t162: f64, t9507: f64, t760: f64, t39818: f64, t39823: f64, t39857: f64, t39859: f64, t39861: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t39960, t39963, t39967, t39989) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1330(t2490, t2494, t2538, t268, t675, t9310, t9314);
        let t40007 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1331(t123, t164, t177, t186, t215, t2492, t2514, t2535, t2549, t2552, t2553, t2554, t2556, t2557, t2591, t268, t39500, t39791, t39795, t39799, t39807, t39813, t39875, t39886, t39960, t39963, t39967, t39989, t730, t745, t9371, t9433, t9454, t9480, t9485, t9530, t9532, t9536);
        let (t40056, t40059, t40067) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1332(t2552, t2555, t2564, t2577, t689, t700);
        let t40072 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1333(t2576, t2581, t2584, t689, t700);
        let t40076 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1334(t2582, t2584, t700, t9305);
        let t40079 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1335(t123, t147, t39500);
        let t40080 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1336(t164, t215, t2491, t2531, t2536, t2539, t2596, t2598, t2601, t2605, t268, t39967, t40056, t40059, t40067, t40072, t40076, t40079, t675, t723, t731, t738, t746, t793, t9367, t9417, t9432, t9435, t9447, t9461, t9469, t9476, t9481, t9488, t9525, t9529, t9533, t9537);
        let (t40082, t40084, t40086, t40088, t40089) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1337(t39913, t39957, t40007, t40080, t158, t162, t2492, t9417, t9507, t760, t39791, t39795, t39799, t39807, t39813, t39818, t39823, t39857, t39859, t39861);
    (t39960, t39963, t39989, t40067, t40072, t40076, t40079, t40082, t40084, t40086, t40088, t40089)
}
