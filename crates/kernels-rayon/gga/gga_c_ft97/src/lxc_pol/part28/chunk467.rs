//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 467/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk467(t370: f64, t7211: f64, t27: f64, t89: f64, t7246: f64, t7250: f64, t7254: f64, t7258: f64) -> (f64, f64, f64) {
    let t7260 = t370 * t7211;
    let t7262 = t89 * t27 * t7260;
    let t7264 = -t7246 / 3.0_f64 + t7250 / 3.0_f64 - t7254 / 6.0_f64 + 2.0_f64 / 3.0_f64 * t7258 - t7262 / 3.0_f64;
    (t7260, t7262, t7264)
}
