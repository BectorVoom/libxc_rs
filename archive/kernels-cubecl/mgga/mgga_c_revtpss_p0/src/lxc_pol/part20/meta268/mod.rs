//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta268 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1117;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta268<F: Float>(t11678: F, t3095: F, t3092: F, t2858: F, t4786: F, t3133: F, t3153: F) -> (F, F, F, F, F) {
        let (t11679, t11680, t11683, t11684, t11687) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1117::<F>(t11678, t3095, t3092, t2858, t4786, t3133, t3153);
    (t11679, t11680, t11683, t11684, t11687)
}
