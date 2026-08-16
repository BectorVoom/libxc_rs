//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 163/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk163(t43: f64, t50: f64, t40: f64, t484: f64, t483: f64, t85: f64, t292: f64, t474: f64, t296: f64, t478: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t44 = t43 <= zeta_threshold;
    let t51 = t50 <= zeta_threshold;
    let t485 = t40 * t484;
    let t486 = t483 * t85;
    let t487 = 0.19751673498613801407e-1_f64 * t486;
    let t490 = piecewise3(t44, 0.0_f64, 2.0_f64 / 3.0_f64 * t292 * t474);
    let t493 = piecewise3(t51, 0.0_f64, 2.0_f64 / 3.0_f64 * t296 * t478);
    let t495 = t490 / 2.0_f64 + t493 / 2.0_f64;
    (t485, t487, t495)
}
