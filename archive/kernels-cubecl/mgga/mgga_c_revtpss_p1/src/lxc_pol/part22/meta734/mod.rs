//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta734 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2794;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta734<F: Float>(t2737: F, t40609: F, t2694: F, t9789: F, t853: F, t9794: F, t775: F, t837: F, t10760: F, t10292: F, t66: F, t240: F) -> (F, F, F, F, F, F, F) {
        let (t40611, t40625, t40627, t40628, t40630, t40633, t40634) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2794::<F>(t2737, t40609, t2694, t9789, t853, t9794, t775, t837, t10760, t10292, t66, t240);
    (t40611, t40625, t40627, t40628, t40630, t40633, t40634)
}
