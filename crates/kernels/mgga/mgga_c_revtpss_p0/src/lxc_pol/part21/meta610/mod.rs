//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta610 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2357;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2358;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2359;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2360;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2361;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta610<F: Float>(t2576: F, t2581: F, t2584: F, t689: F, t700: F, t2582: F, t9305: F, t123: F, t147: F, t39500: F, t164: F, t215: F, t2491: F, t2531: F, t2536: F, t2539: F, t2596: F, t2598: F, t2601: F, t2605: F, t268: F, t39967: F, t40056: F, t40059: F, t40067: F, t675: F, t723: F, t731: F, t738: F, t746: F, t793: F, t9367: F, t9417: F, t9432: F, t9435: F, t9447: F, t9461: F, t9469: F, t9476: F, t9481: F, t9488: F, t9525: F, t9529: F, t9533: F, t9537: F, t39913: F, t39957: F, t40007: F, t158: F, t162: F, t2492: F, t9507: F, t760: F, t2523: F, t9323: F) -> (F, F, F, F, F, F, F, F) {
        let t40072 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2357::<F>(t2576, t2581, t2584, t689, t700);
        let t40076 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2358::<F>(t2582, t2584, t700, t9305);
        let t40079 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2359::<F>(t123, t147, t39500);
        let t40080 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2360::<F>(t164, t215, t2491, t2531, t2536, t2539, t2596, t2598, t2601, t2605, t268, t39967, t40056, t40059, t40067, t40072, t40076, t40079, t675, t723, t731, t738, t746, t793, t9367, t9417, t9432, t9435, t9447, t9461, t9469, t9476, t9481, t9488, t9525, t9529, t9533, t9537);
        let (t40082, t40084, t40086, t40088, t40092) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2361::<F>(t39913, t39957, t40007, t40080, t158, t162, t2492, t9417, t9507, t760, t2523, t9323);
    (t40072, t40076, t40079, t40082, t40084, t40086, t40088, t40092)
}
