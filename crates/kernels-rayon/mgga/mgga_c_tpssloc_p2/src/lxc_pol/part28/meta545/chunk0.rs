//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1811/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1811(t22998: f64, t23185: f64, t81914: f64, t6604: f64, t9971: f64, t23110: f64, t23180: f64, t206: f64, t22723: f64, t268: f64, t23186: f64, t23163: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t82016 = t23185 * t81914 * t22998;
    let t82018 = t6604 * t9971;
    let t82028 = t23185 * t23110 * t23180;
    let t82031 = t22723 * t206 * t268;
    let t82032 = t82031 * t23186;
    let t82038 = t22723 * t23163;
    (t82016, t82018, t82028, t82031, t82032, t82038)
}
