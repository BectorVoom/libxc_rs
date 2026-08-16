//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 824/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk824(t1427: f64, t1670: f64, t1674: f64, t1734: f64, t694: f64, t695: f64, t495: f64, t5403: f64, t192: f64, t301: f64, t1941: f64, t814: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6601 = t1674 * t1670 * t1427;
    let t6604 = t694 * t695 * t1734;
    let t6607 = t694 * t5403 * t495;
    let t6610 = t192 * t1734;
    let t6612 = t1674 * t6610 * t301;
    let t6614 = t1941 * t814;
    (t6601, t6604, t6607, t6610, t6612, t6614)
}
