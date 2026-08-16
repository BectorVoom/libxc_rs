//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 233/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk233(t241: f64, t668: f64, t505: f64, t666: f64, t89: f64, t240: f64) -> (f64, f64, f64, f64, f64) {
    let t669 = t241 * t668;
    let t670 = t669 * t505;
    let t672 = t89 * t666 * t670;
    let t674 = t240 * t240;
    let t675 = 1.0_f64 / t674;
    (t669, t670, t672, t674, t675)
}
