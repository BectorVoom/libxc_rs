//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta349 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1687;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta349<F: Float>(t3133: F, t73: F, t3095: F, t3092: F, t2858: F, t4786: F, t3153: F, t4894: F, t3117: F, t4900: F, t2258: F, t3094: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t11678, t11679, t11680, t11683, t11684, t11687, t11688, t11689, t11692, t11693, t11696) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1687::<F>(t3133, t73, t3095, t3092, t2858, t4786, t3153, t4894, t3117, t4900, t2258, t3094);
    (t11678, t11679, t11680, t11683, t11684, t11687, t11688, t11689, t11692, t11693, t11696)
}
