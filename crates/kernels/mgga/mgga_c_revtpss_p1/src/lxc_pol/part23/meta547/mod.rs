//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta547 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2098;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta547<F: Float>(t13652: F, t177: F, t6800: F, t762: F, t13666: F, t13668: F, t9858: F, t9861: F, t13887: F, t13664: F, t13682: F, t13683: F, t9524: F, t9542: F, t9588: F, t9854: F, t9865: F, t9868: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t22211, t22212, t22213, t22214, t22215, t22216, t22217, t22218, t22219, t22220) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2098::<F>(t13652, t177, t6800, t762, t13666, t13668, t9858, t9861, t13887, t13664, t13682, t13683, t9524, t9542, t9588, t9854, t9865, t9868);
    (t22211, t22212, t22213, t22214, t22215, t22216, t22217, t22218, t22219, t22220)
}
