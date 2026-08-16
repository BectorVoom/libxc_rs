//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta406 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1345;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1346;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta406<F: Float>(t220: F, t40724: F, t2482: F, t2668: F, t823: F, t159: F, t33127: F, t64: F, t222: F, t124: F, t138: F, t40649: F, t9645: F, t810: F, t240: F, t9731: F, t10293: F, t212: F, t800: F, t820: F, t849: F, t9948: F, t2699: F, t2729: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t40725, t40731, t40735, t40737, t40757) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1345::<F>(t220, t40724, t2482, t2668, t823, t159, t33127, t64, t222, t124, t138, t40649, t9645);
        let (t40759, t40763, t40769, t40771, t40781, t40791) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1346::<F>(t40757, t810, t240, t9731, t10293, t124, t212, t800, t820, t849, t9948, t2699, t2729);
    (t40725, t40731, t40735, t40737, t40757, t40759, t40763, t40769, t40771, t40781, t40791)
}
