//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 466/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk466(t7165: f64, t7243: f64, t7238: f64, t7239: f64, t1800: f64, t1317: f64, t28: f64, t469: f64, t7211: f64, t1587: f64, t27: f64, t89: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7244 = t7243 * t7165;
    let t7246 = t7238 * t7239 * t7244;
    let t7248 = t1800 * t7165;
    let t7250 = t1317 * t28 * t7248;
    let t7252 = t469 * t7211;
    let t7254 = t1317 * t28 * t7252;
    let t7256 = t1587 * t7165;
    let t7258 = t89 * t27 * t7256;
    (t7244, t7246, t7248, t7250, t7252, t7254, t7256, t7258)
}
