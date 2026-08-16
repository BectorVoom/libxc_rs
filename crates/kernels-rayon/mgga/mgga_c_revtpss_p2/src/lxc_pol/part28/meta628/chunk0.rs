//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2259/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2259(t5: f64, t101152: f64, t101185: f64, t101225: f64, t101259: f64, t101309: f64, t101340: f64, t101371: f64, t101402: f64, t117: f64, t2014: f64, t25177: f64, t7934: f64) -> (f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t101406 = piecewise3(t8, 0.0_f64, t101152 + t101185 + t101225 + t101259 + t101309 + t101340 + t101371 + t101402);
    let t101407 = t101406 * t117;
    let t101416 = 2.0_f64 * t2014 * t7934 * t25177;
    (t101407, t101416)
}
