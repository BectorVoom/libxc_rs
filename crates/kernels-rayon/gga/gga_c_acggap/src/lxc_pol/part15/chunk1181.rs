//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1181/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1181(t13299: f64, t33944: f64, t40017: f64, t13287: f64, t31195: f64, t39827: f64, t17912: f64, t31443: f64, t39854: f64, t2302: f64, t34823: f64, t8791: f64) -> (f64, f64, f64, f64) {
    let t40450 = t33944 * t13299 * t40017;
    let t40455 = t31195 * t13287 * t39827;
    let t40458 = t31443 * t17912 * t39854;
    let t40465 = t34823 * t13287 * t2302 * t8791;
    (t40450, t40455, t40458, t40465)
}
