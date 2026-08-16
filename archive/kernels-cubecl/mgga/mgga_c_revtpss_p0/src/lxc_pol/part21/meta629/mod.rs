//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta629 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2394;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2395;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta629<F: Float>(t222: F, t40735: F, t10777: F, t10779: F, t2749: F, t40578: F, t10794: F, t10811: F, t10807: F, t10709: F, t10760: F, t9794: F, t124: F, t138: F, t40649: F, t9645: F, t810: F, t10732: F, t240: F, t9731: F, t2664: F, t10293: F, t212: F, t800: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t40737, t40744, t40748, t40750, t40753) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2394::<F>(t222, t40735, t10777, t10779, t2749, t40578, t10794, t10811, t10807, t10709, t10760, t9794);
        let (t40757, t40759, t40761, t40763, t40765, t40769) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2395::<F>(t124, t138, t40649, t9645, t810, t10732, t10760, t9794, t240, t9731, t2664, t10293, t212, t800);
    (t40737, t40744, t40748, t40750, t40753, t40757, t40759, t40761, t40763, t40765, t40769)
}
