//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta716 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2750;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2751;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta716<F: Float>(t234: F, t251: F, t268: F, t39644: F, t8779: F, t39497: F, t874: F, t875: F, t2718: F, t2760: F, t10530: F, t2723: F, t39583: F, t10535: F, t231: F, t281: F, t624: F, t836: F, t2722: F, t68: F, t10529: F, t2453: F, t10960: F, t2435: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t39649, t39652, t39656, t39662) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2750::<F>(t234, t251, t268, t39644, t8779, t39497, t874, t875, t2718, t2760, t10530, t2723, t39583);
        let (t39673, t39678, t39680, t39683, t39687) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2751::<F>(t10535, t231, t281, t624, t836, t2722, t68, t10529, t2453, t2723, t10960, t2435);
    (t39649, t39652, t39656, t39662, t39673, t39678, t39680, t39683, t39687)
}
