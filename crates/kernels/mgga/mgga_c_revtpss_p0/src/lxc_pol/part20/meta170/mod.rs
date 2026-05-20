//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta170 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk903;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk904;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta170<F: Float>(t730: F, t9446: F, t2596: F, t675: F, t215: F, t723: F, t2553: F, t738: F, t2491: F, t177: F, t9417: F, t2495: F, t9368: F, t2531: F, t2536: F, t2539: F, t2549: F, t2557: F, t2591: F, t2598: F, t2601: F, t2605: F, t268: F, t724: F, t731: F, t746: F, t9278: F, t9308: F, t9316: F, t9329: F, t9333: F, t9433: F, t9435: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t9447, t9450, t9454, t9461, t9469, t9476, t9480, t9481) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk903::<F>(t730, t9446, t2596, t675, t215, t723, t2553, t738, t2491, t177, t9417, t2495, t9368);
        let t9484 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk904::<F>(t2531, t2536, t2539, t2549, t2557, t2591, t2598, t2601, t2605, t268, t675, t724, t731, t746, t9278, t9308, t9316, t9329, t9333, t9433, t9435, t9447, t9450, t9454, t9461, t9469, t9476, t9480, t9481);
    (t9447, t9450, t9454, t9461, t9469, t9476, t9480, t9481, t9484)
}
