//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 752/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk752(t33489: f64, t762: f64, t263: f64, t7440: f64, t684: f64, t9770: f64, t7436: f64, t92: f64) -> (f64, f64, f64, f64) {
    let t33490 = t762 * t33489;
    let t33494 = t7440 * t263;
    let t33496 = t9770 * t33494 * t684;
    let t33499 = t7436 * t92;
    (t33490, t33494, t33496, t33499)
}
