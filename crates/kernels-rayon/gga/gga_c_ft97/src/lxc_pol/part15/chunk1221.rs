//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1221/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1221(t299: f64, t91432: f64, t91469: f64, t13: f64, t20494: f64, t21100: f64, t21794: f64, t22488: f64, t86576: f64, t88053: f64, t89765: f64) -> f64 {
    let t300 = 10000000.0_f64 <= t299;
    let t91471 = piecewise3(t300, 0.0_f64, t91432 + t91469);
    let tv4rho44 = 4.0_f64 * t20494 + 4.0_f64 * t21100 + 4.0_f64 * t21794 + 4.0_f64 * t22488 + t13 * (t86576 + t88053 + t89765 + t91471);
    tv4rho44
}
