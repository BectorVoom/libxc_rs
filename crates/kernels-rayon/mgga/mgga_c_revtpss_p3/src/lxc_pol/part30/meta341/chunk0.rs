//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1355/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1355(t2439: f64, t931: f64, t2915: f64, t698: f64, t2922: f64, t913: f64, t275: f64, t290: f64, t2925: f64, t2935: f64, t945: f64, t2967: f64, t941: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11366 = t2439 * t931;
    let t11368 = t698 * t2915;
    let t11384 = 1.0_f64 / t2922 / t913;
    let t11385 = t275 * t11384;
    let t11387 = 1.0_f64 / t2925 / t290;
    let t11399 = t2935 * t945;
    let t11404 = t941 * t2967;
    (t11366, t11368, t11385, t11387, t11399, t11404)
}
