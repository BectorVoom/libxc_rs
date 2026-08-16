//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 1054/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk1054(t11299: f64, t11292: f64, t11287: f64, t11280: f64, t8601: f64, t8613: f64, t11609: f64, t1611: f64, t1617: f64, t3721: f64, t4915: f64, t11279: f64, t575: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t33098 = 4.0_f64 * t11299;
    let t33099 = 12.0_f64 * t11292;
    let t33100 = 4.0_f64 * t11287;
    let t33101 = 2.0_f64 * t11280;
    let t33103 = 8.0_f64 * t8601 * t8613;
    let t33105 = 2.0_f64 * t1611 * t11609;
    let t33110 = 6.0_f64 * t4915 * t3721 * t1617;
    let t33111 = t11279 * t575;
    (t33098, t33099, t33100, t33101, t33103, t33105, t33110, t33111)
}
