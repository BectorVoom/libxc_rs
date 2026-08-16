//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 905/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk905(t1995: f64, t8851: f64, t527: f64, t23: f64, t32905: f64, t153: f64, t1984: f64, t22: f64, t36452: f64, t37991: f64, t355: f64, t7368: f64) -> (f64, f64, f64, f64, f64) {
    let t40087 = t1995 * t8851;
    let t40227 = t527 * t8851;
    let t40266 = t23 * t32905;
    let t40280 = 1.0_f64 / t153 / t37991 / t22 / t1984 / t36452 / 96.0_f64;
    let t40424 = t355 * t7368;
    (t40087, t40227, t40266, t40280, t40424)
}
