//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 990/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk990(t22441: f64, t681: f64, t89: f64, t1882: f64, t22218: f64, t22222: f64, t22183: f64, t8392: f64, t22188: f64, t22402: f64, t22261: f64, t22373: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t84625 = t89 * t681 * t22441;
    let t84628 = t1882 * t22218;
    let t84630 = t1882 * t22222;
    let t84697 = t8392 * t22183;
    let t84734 = t8392 * t22188;
    let t84740 = t1882 * t22402;
    let t84767 = t1882 * t22261;
    let t84795 = t8392 * t22373;
    (t84625, t84628, t84630, t84697, t84734, t84740, t84767, t84795)
}
