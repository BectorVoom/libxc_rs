//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 978/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk978(t21839: f64, t2253: f64, t21878: f64, t8675: f64, t21881: f64, t21863: f64, t21850: f64, t668: f64, t21852: f64, t21885: f64, t21895: f64, t21871: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t82267 = t2253 * t21839;
    let t82303 = t8675 * t21878;
    let t82326 = t8675 * t21881;
    let t82328 = t8675 * t21863;
    let t82342 = t21850 * t668;
    let t82361 = t2253 * t21852;
    let t82367 = t8675 * t21885;
    let t82405 = t8675 * t21895;
    let t82407 = t8675 * t21871;
    (t82267, t82303, t82326, t82328, t82342, t82361, t82367, t82405, t82407)
}
