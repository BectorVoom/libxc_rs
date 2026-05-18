//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1120/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1120<F: Float>(t119757: F, t31846: F, t4451: F, t119752: F, t120097: F, t4367: F, t119821: F, t31753: F, t4486: F, t827: F, t828: F, t8478: F) -> (F, F, F) {
    let t126052 = t31846 * t119757 * t4451;
    let t126055 = t120097 * t119752 * t4367;
    let t126062 = t8478 * t119821 * t31753 * t827 * t828 * t4486;
    (t126052, t126055, t126062)
}
