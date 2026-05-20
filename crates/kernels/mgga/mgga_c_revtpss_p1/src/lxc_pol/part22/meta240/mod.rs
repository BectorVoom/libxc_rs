//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta240 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1496;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1497;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta240<F: Float>(t114: F, t5915: F, t655: F, t2335: F, t4261: F, t5892: F, t69: F, t508: F, t4303: F, t4306: F, t2498: F, t2518: F, t2522: F, t2562: F, t2569: F, t2579: F, t2587: F, t2610: F, t2628: F, t2632: F) -> (F, F, F, F, F, F) {
        let (t5916, t5920) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1496::<F>(t114, t5915, t655, t2335, t4261, t5892, t69);
        let (t5921, t5924, t5925, t5926) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1497::<F>(t508, t5920, t4303, t4306, t2498, t2518, t2522, t2562, t2569, t2579, t2587, t2610, t2628, t2632);
    (t5916, t5920, t5921, t5924, t5925, t5926)
}
