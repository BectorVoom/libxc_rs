//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 811/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk811(t2842: f64, t7672: f64, t684: f64, t2881: f64, t312: f64, t7611: f64, t2874: f64, t7679: f64, t870: f64, t1501: f64, t25253: f64, t296: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t34197 = t2842 * t7672;
    let t34198 = t34197 * t684;
    let t34199 = t2881 * t34198;
    let t34202 = t312 * t7611;
    let t34203 = t34202 * t684;
    let t34204 = t2874 * t34203;
    let t34207 = t870 * t7679;
    let t34208 = t34207 * t684;
    let t34209 = t2881 * t34208;
    let t34212 = t25253 * t1501;
    let t34213 = t296 * t34212;
    (t34197, t34198, t34199, t34202, t34203, t34204, t34207, t34208, t34209, t34212, t34213)
}
