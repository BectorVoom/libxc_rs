//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 155/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk155<F: Float>(t624: F, t64: F, t44: F, t49: F, t56: F, t614: F, t617: F, t620: F, t38: F, t45: F, t78: F, t57: F, t81: F, t606: F) -> (F, F, F, F, F, F, F, F, F) {
    let t625 = t64 * t624;
    let t626 = 8.0 / 3.0 * t625;
    let t627 = -8.0 / 3.0 * t614 * t49 + 5.0 / 6.0 * t44 * t617 - 5.0 / 6.0 * t56 * t620 + t626;
    let t628 = t38 * t627;
    let t631 = t45 * t45;
    let t633 = 1.0 / t78 / t631;
    let t635 = t57 * t57;
    let t637 = 1.0 / t81 / t635;
    let t640 = -4.0 / 3.0 * t633 * t606 + 4.0 / 3.0 * t637 * t606;
    (t625, t626, t627, t628, t631, t633, t635, t637, t640)
}
