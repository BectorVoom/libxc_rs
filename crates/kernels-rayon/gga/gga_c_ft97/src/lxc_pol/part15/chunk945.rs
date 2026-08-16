//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 945/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk945(t11262: f64, t1526: f64, t19950: f64, t19965: f64, t342: f64, t630: f64, t19961: f64, t7705: f64, t19957: f64, t21048: f64, t8675: f64, t21025: f64, t358: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t75881 = t1526 * t11262 * t19950;
    let t75935 = t342 * t630 * t19965;
    let t75944 = t1526 * t7705 * t19961;
    let t75947 = t1526 * t7705 * t19957;
    let t75994 = t8675 * t21048;
    let t75996 = t21025 * t358;
    (t75881, t75935, t75944, t75947, t75994, t75996)
}
