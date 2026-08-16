//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta221 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk858;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk859;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk860;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta221<F: Float>(t2349: F, t5895: F, t100: F, t5823: F, t1479: F, t1509: F, t2357: F, t108: F, t105: F, t109: F, t1507: F, t1510: F, t97: F, tau1: F, t114: F, t655: F, t2335: F, t4261: F, t5892: F, t69: F, t508: F, t4303: F, t4306: F, t2498: F, t2518: F, t2522: F, t2562: F, t2569: F, t2579: F, t2587: F, t2610: F, t2628: F, t2632: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t5896, t5899, t5902, t5907, t5911, t5915) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk858::<F>(t2349, t5895, t100, t5823, t1479, t1509, t2357, t108, t105, t109, t1507, t1510, t97, tau1);
        let (t5916, t5920) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk859::<F>(t114, t5915, t655, t2335, t4261, t5892, t69);
        let (t5921, t5924, t5925, t5926) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk860::<F>(t508, t5920, t4303, t4306, t2498, t2518, t2522, t2562, t2569, t2579, t2587, t2610, t2628, t2632);
    (t5896, t5899, t5902, t5907, t5911, t5915, t5916, t5920, t5921, t5924, t5925, t5926)
}
