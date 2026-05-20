//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta726 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2782;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta726<F: Float>(t40169: F, t760: F, t2523: F, t9372: F, t39909: F, t738: F, t745: F, t2251: F, t2609: F, t2611: F, t36: F, t716: F) -> (F, F, F, F, F, F) {
        let (t40171, t40172, t40182, t40184, t40186, t40188) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2782::<F>(t40169, t760, t2523, t9372, t39909, t738, t745, t2251, t2609, t2611, t36, t716);
    (t40171, t40172, t40182, t40184, t40186, t40188)
}
