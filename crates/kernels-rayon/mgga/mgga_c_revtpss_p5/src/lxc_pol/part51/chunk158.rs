//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 158/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk158(t607: f64, t70: f64, t39: f64, t41: f64, t48: f64, t606: f64, t60: f64, t579: f64, t66: f64, t64: f64, t44: f64, t49: f64, t56: f64, rho0: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t608 = t607 * t70;
    let t611 = t39 * rho0;
    let t613 = 1.0_f64 / t41 / t611;
    let t614 = sigma0 * t613;
    let t617 = t48 * t606;
    let t620 = t60 * t606;
    let t624 = 1.0_f64 / t66 / t579;
    let t625 = t64 * t624;
    let t626 = 8.0_f64 / 3.0_f64 * t625;
    let t627 = -8.0_f64 / 3.0_f64 * t614 * t49 + 5.0_f64 / 6.0_f64 * t44 * t617 - 5.0_f64 / 6.0_f64 * t56 * t620 + t626;
    (t608, t613, t614, t620, t624, t625, t626, t627)
}
