//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta440 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1396;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta440<F: Float>(t1340: F, t40196: F, t40192: F, t40113: F, t40169: F, t40135: F, t3869: F, t39739: F, t39430: F, t39742: F, t39440: F, t39532: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t47088, t47092, t47096, t47098, t47109, t47116, t47118, t47122, t47124, t47131) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1396::<F>(t1340, t40196, t40192, t40113, t40169, t40135, t3869, t39739, t39430, t39742, t39440, t39532);
    (t47088, t47092, t47096, t47098, t47109, t47116, t47118, t47122, t47124, t47131)
}
