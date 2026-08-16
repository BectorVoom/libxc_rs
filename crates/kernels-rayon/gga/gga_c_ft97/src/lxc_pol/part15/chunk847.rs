//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 847/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk847(t22432: f64, t22438: f64, t295: f64, t312: f64, t1255: f64, t5299: f64, t840: f64, t22161: f64, t319: f64, t1212: f64, t5424: f64, t1091: f64, t19576: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22439 = t22432 + t22438;
    let t22441 = t295 * t22439 * t312;
    let t22446 = t840 * t1255 * t5299;
    let t22449 = t840 * t319 * t22161;
    let t22454 = t840 * t5424 * t1212;
    let t22456 = t19576 * t1091;
    (t22439, t22441, t22446, t22449, t22454, t22456)
}
