//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta738 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2799;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2800;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta738<F: Float>(t10782: F, t40731: F, t159: F, t33127: F, t64: F, t222: F, t10709: F, t10760: F, t9794: F, t124: F, t138: F, t40649: F, t9645: F, t810: F, t10732: F, t240: F, t9731: F, t2664: F, t10293: F, t212: F, t800: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t40732, t40735, t40737, t40753, t40757) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2799::<F>(t10782, t40731, t159, t33127, t64, t222, t10709, t10760, t9794, t124, t138, t40649, t9645);
        let (t40759, t40761, t40763, t40765, t40769) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2800::<F>(t40757, t810, t10732, t10760, t9794, t240, t9731, t2664, t10293, t124, t212, t800);
    (t40732, t40735, t40737, t40753, t40757, t40759, t40761, t40763, t40765, t40769)
}
