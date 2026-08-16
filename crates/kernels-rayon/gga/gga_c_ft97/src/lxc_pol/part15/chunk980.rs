//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 980/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk980(t1882: f64, t22348: f64, t22446: f64, t1775: f64, t22323: f64, t22287: f64, t22294: f64, t1196: f64, t283: f64, t21249: f64, t21253: f64, t280: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t82630 = t1882 * t22348;
    let t82638 = t1882 * t22446;
    let t82769 = t1775 * t22323;
    let t82771 = t1775 * t22287;
    let t82773 = t1775 * t22294;
    let t82816 = t1196 * t283;
    let t82845 = t280 * t21249 * t21253;
    (t82630, t82638, t82769, t82771, t82773, t82816, t82845)
}
