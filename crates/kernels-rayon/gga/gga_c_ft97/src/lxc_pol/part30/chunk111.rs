//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 111/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk111(t172: f64, t209: f64, t231: f64, t228: f64, t227: f64, t9: f64) -> (f64, f64, f64, f64) {
    let t696 = t209 * t172;
    let t697 = t696 * t231;
    let t698 = t228 * t697;
    let t699 = 0.6384360837962962963e-2_f64 * t698;
    let t701 = t9 * t227 * t209;
    (t696, t698, t699, t701)
}
