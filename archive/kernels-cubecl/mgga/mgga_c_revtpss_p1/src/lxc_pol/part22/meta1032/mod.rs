//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1032 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3615;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1032<F: Float>(t12254: F, t141: F, t68345: F, t43764: F, t68308: F, t1145: F, t68295: F, t20349: F, t698: F, t20352: F, t68299: F, t68303: F) -> (F, F, F, F, F, F, F) {
        let (t68529, t68532, t68535, t68538, t68540, t68543, t68546) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3615::<F>(t12254, t141, t68345, t43764, t68308, t1145, t68295, t20349, t698, t20352, t68299, t68303);
    (t68529, t68532, t68535, t68538, t68540, t68543, t68546)
}
