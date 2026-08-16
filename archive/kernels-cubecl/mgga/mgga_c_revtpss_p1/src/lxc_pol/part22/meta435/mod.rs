//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta435 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2063;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta435<F: Float>(t221: F, t4433: F, t10703: F, t2674: F, t4353: F, t9794: F, t10760: F, t10890: F, t1549: F, t1544: F, t2430: F, t2477: F, t828: F) -> (F, F, F, F, F, F) {
        let (t14757, t14759, t14761, t14765, t14767, t14769) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2063::<F>(t221, t4433, t10703, t2674, t4353, t9794, t10760, t10890, t1549, t1544, t2430, t2477, t828);
    (t14757, t14759, t14761, t14765, t14767, t14769)
}
