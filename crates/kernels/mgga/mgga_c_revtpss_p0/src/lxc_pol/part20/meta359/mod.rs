//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta359 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1305;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1306;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1307;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1308;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta359<F: Float>(t234: F, t251: F, t268: F, t39644: F, t8779: F, t39497: F, t874: F, t875: F, t2718: F, t2760: F, t10530: F, t2723: F, t39583: F, t10657: F, t2646: F, t2724: F, t39622: F, t39624: F, t39629: F, t39633: F, t39635: F, t39640: F, t820: F, t231: F, t2798: F, t39599: F, t10535: F, t281: F, t624: F, t836: F, t2722: F, t68: F, t10529: F, t2453: F, t10523: F, t10542: F, t10960: F, t2435: F, t2482: F, t39620: F, t686: F, t72: F, t879: F) -> (F, F, F, F, F, F, F, F) {
        let (t39649, t39652, t39656, t39662) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1305::<F>(t234, t251, t268, t39644, t8779, t39497, t874, t875, t2718, t2760, t10530, t2723, t39583);
        let t39664 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1306::<F>(t10657, t2646, t2724, t39622, t39624, t39629, t39633, t39635, t39640, t39649, t39652, t39656, t39662, t820);
        let (t39668, t39673, t39675, t39678, t39680) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1307::<F>(t231, t268, t2798, t39599, t10535, t281, t624, t836, t2722, t68, t10529, t2453);
        let (t39683, t39685, t39687, t39692) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1308::<F>(t2723, t281, t39675, t39680, t10523, t10542, t10960, t2435, t2482, t39620, t686, t72, t879);
    (t39664, t39668, t39673, t39678, t39683, t39685, t39687, t39692)
}
