//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 708/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk708(t1882: f64, t7111: f64, t7038: f64, t28496: f64, t2862: f64, t319: f64, t7021: f64, t875: f64, t840: f64, t871: f64, t2749: f64, t7045: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t29285 = t1882 * t7111;
    let t29287 = t1882 * t7038;
    let t29290 = t2862 * t319 * t28496;
    let t29293 = t7021 * t875;
    let t29295 = t840 * t871 * t29293;
    let t29299 = t840 * t2749 * t7045;
    (t29285, t29287, t29290, t29293, t29295, t29299)
}
