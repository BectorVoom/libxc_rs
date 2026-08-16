//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1168/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1168(t2001: f64, t5561: f64, t5946: f64, t1755: f64, t30644: f64, t5792: f64, t7822: f64, t13287: f64, t31443: f64, t39858: f64, t2297: f64, t8406: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40105 = t2001 * t5561;
    let t40107 = t2001 * t5946;
    let t40109 = t30644 * t1755;
    let t40111 = t7822 * t5792;
    let t40114 = t31443 * t13287 * t39858;
    let t40116 = t2297 * t8406;
    (t40105, t40107, t40109, t40111, t40114, t40116)
}
