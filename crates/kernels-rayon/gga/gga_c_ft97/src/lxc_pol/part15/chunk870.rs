//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 870/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk870(t143: f64, t37406: f64, t37352: f64, t2: f64, t32905: f64, t355: f64, t7368: f64, t525: f64, t7760: f64, t1554: f64, t1984: f64, t11176: f64, t151: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t40294 = t143 * t37406;
    let t40337 = t37352 * t143;
    let t40379 = t32905 * t2;
    let t40424 = t355 * t7368;
    let t40425 = t40424 * t2;
    let t40436 = t7760 * t525;
    let t40437 = t40436 * t2;
    let t40465 = t1554 * t1984;
    let t40466 = t40465 * t2;
    let t40485 = 280.0_f64 / 81.0_f64 * t11176 * t151;
    (t40294, t40337, t40379, t40424, t40425, t40436, t40437, t40465, t40466, t40485)
}
