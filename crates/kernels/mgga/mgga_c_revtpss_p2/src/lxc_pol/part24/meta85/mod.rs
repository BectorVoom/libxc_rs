//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta85 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk501;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk502;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk503;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk504;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk505;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta85<F: Float>(t675: F, t738: F, t182: F, t737: F, t177: F, t2492: F, t745: F, t2514: F, t2491: F, t2495: F, t123: F, t173: F, t186: F, t2434: F, t2522: F, t2531: F, t2537: F, t2539: F, t2549: F, t2554: F, t2557: F, t2562: F, t2569: F, t2579: F, t2587: F, t268: F, t724: F, t731: F, t739: F, t746: F, t162: F, t158: F, t157: F, t37: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2591, t2595, t2596) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk501::<F>(t675, t738, t182, t737);
        let (t2597, t2598) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk502::<F>(t177, t2596, t2492, t745);
        let (t2601, t2604, t2605, t2608) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk503::<F>(t2514, t745, t177, t2491, t2492, t2495, t123, t173, t186, t2434, t2522, t2531, t2537, t2539, t2549, t2554, t2557, t2562, t2569, t2579, t2587, t2591, t2597, t2598, t268, t724, t731, t739, t746);
        let t2609 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk504::<F>(t162, t2608);
        let (t2610, t2611) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk505::<F>(t158, t2609, t157, t37);
    (t2591, t2595, t2596, t2597, t2598, t2601, t2604, t2605, t2608, t2609, t2610, t2611)
}
