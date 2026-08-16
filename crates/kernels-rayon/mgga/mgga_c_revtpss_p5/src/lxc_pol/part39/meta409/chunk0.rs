//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1486/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1486(t2681: f64, t64: f64, t10207: f64, t111: f64, t116: f64, t13424: f64, t1501: f64, t2371: f64, t4245: f64, t670: f64, t1518: f64, t2319: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t46089 = t64 * t2681;
    let t46157 = 1.0_f64 / t10207 / t111;
    let t49686 = t13424 * t116;
    let t75485 = t1501 * t2371;
    let t75667 = t4245 * t670;
    let t98484 = t2319 * t1518;
    (t46089, t46157, t49686, t75485, t75667, t98484)
}
