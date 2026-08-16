//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 912/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk912(t38953: f64, t4829: f64, t4747: f64, t8232: f64, t4833: f64, t4790: f64, t582: f64, t4739: f64, t4807: f64, t2101: f64, t4824: f64, t4726: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t63187 = t38953 * t4829;
    let t63219 = t8232 * t4747;
    let t63225 = t8232 * t4833;
    let t63258 = t582 * t4790;
    let t63530 = t8232 * t4739;
    let t63536 = t8232 * t4807;
    let t63586 = t2101 * t4790;
    let t63613 = t38953 * t4824;
    let t63746 = t8232 * t4726;
    (t63187, t63219, t63225, t63258, t63530, t63536, t63586, t63613, t63746)
}
