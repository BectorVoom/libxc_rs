//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 156/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk156<F: Float>(t607: F, t70: F, t39: F, t41: F, t48: F, t606: F, t60: F, t579: F, t66: F, rho0: F, sigma0: F) -> (F, F, F, F, F, F) {
    let t608 = t607 * t70;
    let t611 = t39 * rho0;
    let t613 = F::cast_from(1.0_f64) / t41 / t611;
    let t614 = sigma0 * t613;
    let t617 = t48 * t606;
    let t620 = t60 * t606;
    let t624 = F::cast_from(1.0_f64) / t66 / t579;
    (t608, t613, t614, t617, t620, t624)
}
