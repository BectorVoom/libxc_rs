//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta301 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1551;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1552;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta301<F: Float>(t10605: F, t2612: F, t2523: F, t2626: F, t760: F, t9425: F, t2609: F, t606: F, t706: F, t10592: F, t10594: F, t10596: F, t10598: F, t10602: F, t10604: F, t9542: F, t10550: F, t10571: F, t10590: F, t225: F, t2475: F, t73: F, t2394: F, t775: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t10607, t10608, t10609, t10611, t10612, t10613, t10614, t10615) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1551::<F>(t10605, t2612, t2523, t2626, t760, t9425, t2609, t606, t706, t10592, t10594, t10596, t10598, t10602, t10604, t9542);
        let (t10618, t10626, t10627) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1552::<F>(t10550, t10571, t10590, t10615, t225, t2475, t73, t2394, t775);
    (t10607, t10608, t10609, t10611, t10612, t10613, t10614, t10618, t10626, t10627)
}
