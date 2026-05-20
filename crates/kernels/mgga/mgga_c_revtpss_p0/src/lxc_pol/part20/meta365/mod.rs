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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1330;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1331;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1332;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1333;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1334;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1335;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1336;
use chunk7::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1337;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta365<F: Float>(t2490: F, t2494: F, t2538: F, t268: F, t675: F, t9310: F, t9314: F, t123: F, t164: F, t177: F, t186: F, t215: F, t2492: F, t2514: F, t2535: F, t2549: F, t2552: F, t2553: F, t2554: F, t2556: F, t2557: F, t2591: F, t39500: F, t39791: F, t39795: F, t39799: F, t39807: F, t39813: F, t39875: F, t39886: F, t730: F, t745: F, t9371: F, t9433: F, t9454: F, t9480: F, t9485: F, t9530: F, t9532: F, t9536: F, t2555: F, t2564: F, t2577: F, t689: F, t700: F, t2576: F, t2581: F, t2584: F, t2582: F, t9305: F, t147: F, t2491: F, t2531: F, t2536: F, t2539: F, t2596: F, t2598: F, t2601: F, t2605: F, t723: F, t731: F, t738: F, t746: F, t793: F, t9367: F, t9417: F, t9432: F, t9435: F, t9447: F, t9461: F, t9469: F, t9476: F, t9481: F, t9488: F, t9525: F, t9529: F, t9533: F, t9537: F, t39913: F, t39957: F, t158: F, t162: F, t9507: F, t760: F, t39818: F, t39823: F, t39857: F, t39859: F, t39861: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t39960, t39963, t39967, t39989) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1330::<F>(t2490, t2494, t2538, t268, t675, t9310, t9314);
        let t40007 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1331::<F>(t123, t164, t177, t186, t215, t2492, t2514, t2535, t2549, t2552, t2553, t2554, t2556, t2557, t2591, t268, t39500, t39791, t39795, t39799, t39807, t39813, t39875, t39886, t39960, t39963, t39967, t39989, t730, t745, t9371, t9433, t9454, t9480, t9485, t9530, t9532, t9536);
        let (t40056, t40059, t40067) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1332::<F>(t2552, t2555, t2564, t2577, t689, t700);
        let t40072 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1333::<F>(t2576, t2581, t2584, t689, t700);
        let t40076 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1334::<F>(t2582, t2584, t700, t9305);
        let t40079 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1335::<F>(t123, t147, t39500);
        let t40080 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1336::<F>(t164, t215, t2491, t2531, t2536, t2539, t2596, t2598, t2601, t2605, t268, t39967, t40056, t40059, t40067, t40072, t40076, t40079, t675, t723, t731, t738, t746, t793, t9367, t9417, t9432, t9435, t9447, t9461, t9469, t9476, t9481, t9488, t9525, t9529, t9533, t9537);
        let (t40082, t40084, t40086, t40088, t40089) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1337::<F>(t39913, t39957, t40007, t40080, t158, t162, t2492, t9417, t9507, t760, t39791, t39795, t39799, t39807, t39813, t39818, t39823, t39857, t39859, t39861);
    (t39960, t39963, t39989, t40067, t40072, t40076, t40079, t40082, t40084, t40086, t40088, t40089)
}
