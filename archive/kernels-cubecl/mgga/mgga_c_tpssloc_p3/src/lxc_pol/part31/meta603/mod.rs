//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta603 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1848;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta603<F: Float>(t87583: F, t87601: F, t87603: F, t87612: F, t87618: F, t87668: F, t87679: F, t87709: F, t87714: F, t87729: F, t87733: F, t87753: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t92739, t92749, t92754, t92760, t92768, t92795, t92798, t92810, t92811, t92822, t92825, t92846) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1848::<F>(t87583, t87601, t87603, t87612, t87618, t87668, t87679, t87709, t87714, t87729, t87733, t87753);
    (t92739, t92749, t92754, t92760, t92768, t92795, t92798, t92810, t92811, t92822, t92825, t92846)
}
