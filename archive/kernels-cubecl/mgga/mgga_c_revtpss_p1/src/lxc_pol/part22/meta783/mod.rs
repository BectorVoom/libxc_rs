//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta783 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2873;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta783<F: Float>(t17847: F, t3588: F, t17854: F, t1209: F, t17887: F, t12657: F, t3754: F, t12722: F, t3555: F, t12640: F, t3552: F, t3766: F) -> (F, F, F, F, F, F, F) {
        let (t45675, t45679, t45683, t45697, t45700, t45707, t45710) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2873::<F>(t17847, t3588, t17854, t1209, t17887, t12657, t3754, t12722, t3555, t12640, t3552, t3766);
    (t45675, t45679, t45683, t45697, t45700, t45707, t45710)
}
