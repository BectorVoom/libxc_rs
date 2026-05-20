//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta77 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk461;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk462;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta77<F: Float>(t100: F, t1504: F, t55: F, t108: F, t105: F, t109: F, t97: F, tau1: F, t114: F, t655: F, t653: F, t69: F) -> (F, F, F, F, F, F) {
        let (t1505, t1507, t1509, t1513) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk461::<F>(t100, t1504, t55, t108, t105, t109, t97, tau1);
        let (t1514, t1518) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk462::<F>(t114, t1513, t655, t653, t69);
    (t1505, t1507, t1509, t1513, t1514, t1518)
}
