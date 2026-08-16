//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 1025/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk1025(t19298: f64, t19301: f64, t19304: f64, t10797: f64, t19273: f64, t19276: f64, t19283: f64, t19287: f64, t19292: f64, t19295: f64, t19852: f64, t19826: f64, t19836: f64, t19849: f64) -> f64 {
    let t19857 = t19298 / 9.0_f64;
    let t19858 = 2.0_f64 / 9.0_f64 * t19301;
    let t19859 = 2.0_f64 / 27.0_f64 * t19304;
    let t19860 = 2.0_f64 / 9.0_f64 * t19273 + 4.0_f64 / 3.0_f64 * t19276 - t19852 - t10797 + 2.0_f64 * t19283 - t19287 / 3.0_f64 - 6.0_f64 * t19292 + 4.0_f64 * t19295 + t19857 - t19858 + t19859;
    let t19862 = t19826 + t19836 + t19849 + t19860;
    t19862
}
