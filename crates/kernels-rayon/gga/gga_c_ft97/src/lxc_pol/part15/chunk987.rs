//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 987/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk987(t1882: f64, t22352: f64, t21978: f64, t312: f64, t22449: f64, t22230: f64, t22407: f64, t22361: f64, t22465: f64, t22214: f64, t22196: f64, t22457: f64, t8392: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t83808 = t1882 * t22352;
    let t83825 = t312 * t21978;
    let t83982 = t1882 * t22449;
    let t83988 = t1882 * t22230;
    let t83990 = t1882 * t22407;
    let t84080 = t1882 * t22361;
    let t84087 = t1882 * t22465;
    let t84138 = t1882 * t22214;
    let t84167 = t1882 * t22196;
    let t84169 = t8392 * t22457;
    (t83808, t83825, t83982, t83988, t83990, t84080, t84087, t84138, t84167, t84169)
}
