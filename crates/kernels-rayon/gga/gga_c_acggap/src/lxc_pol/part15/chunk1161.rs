//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1161/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1161(t13364: f64, t31195: f64, t38850: f64, t1988: f64, t9687: f64, t2001: f64, t6361: f64, t5561: f64, t5946: f64, t1755: f64, t30644: f64, t5792: f64, t7822: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t40095 = t31195 * t13364 * t38850;
    let t40099 = t1988 * t9687;
    let t40101 = t2001 * t6361;
    let t40105 = t2001 * t5561;
    let t40107 = t2001 * t5946;
    let t40109 = t30644 * t1755;
    let t40111 = t7822 * t5792;
    (t40095, t40099, t40101, t40105, t40107, t40109, t40111)
}
