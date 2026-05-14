//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 160/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk160<F: Float>(t607: F, t70: F, t39: F, t41: F, t48: F, t606: F, t60: F, t579: F, t66: F, t64: F, t44: F, t49: F, t56: F, t38: F, t45: F, t78: F, rho0: F, sigma0: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t608 = t607 * t70;
    let t611 = t39 * rho0;
    let t613 = 1.0 / t41 / t611;
    let t614 = sigma0 * t613;
    let t617 = t48 * t606;
    let t620 = t60 * t606;
    let t624 = 1.0 / t66 / t579;
    let t625 = t64 * t624;
    let t626 = 8.0 / 3.0 * t625;
    let t627 = -8.0 / 3.0 * t614 * t49 + 5.0 / 6.0 * t44 * t617 - 5.0 / 6.0 * t56 * t620 + t626;
    let t628 = t38 * t627;
    let t631 = t45 * t45;
    let t633 = 1.0 / t78 / t631;
    (t608, t613, t614, t620, t624, t625, t626, t627, t628, t631, t633)
}
