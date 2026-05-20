//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta725 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2781;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta725<F: Float>(t40135: F, t760: F, t10565: F, t606: F, t706: F, t717: F, t10587: F, t2496: F, t39875: F, t39894: F, t9371: F, t39960: F, t39963: F) -> (F, F, F, F, F, F, F) {
        let (t40137, t40139, t40150, t40156, t40165, t40167, t40169) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2781::<F>(t40135, t760, t10565, t606, t706, t717, t10587, t2496, t39875, t39894, t9371, t39960, t39963);
    (t40137, t40139, t40150, t40156, t40165, t40167, t40169)
}
