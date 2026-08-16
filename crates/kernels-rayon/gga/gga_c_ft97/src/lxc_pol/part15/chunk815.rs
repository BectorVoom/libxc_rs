//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 815/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk815(t21953: f64, t89: f64, t9716: f64, t10398: f64, t14715: f64, t14895: f64, t19246: f64, t19249: f64, t19298: f64, t19301: f64, t19304: f64, t21947: f64, t21951: f64) -> (f64, f64) {
    let t21955 = t89 * t9716 * t21953;
    let t21957 = t19246 / 6.0_f64 - t19249 / 3.0_f64 + t19298 / 18.0_f64 - t19301 / 9.0_f64 + t19304 / 27.0_f64 - 2.0_f64 / 9.0_f64 * t14895 - 2.0_f64 / 27.0_f64 * t14715 - t21947 / 3.0_f64 - t21951 / 3.0_f64 - t10398 - 5.0_f64 / 81.0_f64 * t21955;
    (t21955, t21957)
}
