//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 892/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk892(t1035: f64, t1979: f64, t355: f64, t864: f64, t368: f64, t7458: f64, t7709: f64, t7799: f64, t381: f64, t7636: f64, t7461: f64, t7637: f64, t7770: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t30572 = t1035 * t1979;
    let t30573 = t355 * t864;
    let t30576 = t30572 * t7458 * t368 * t30573;
    let t30582 = t7799 * t7709;
    let t30589 = t381 * t7636;
    let t30590 = t30589 * t7461;
    let t30592 = t7637 * t7770;
    (t30572, t30573, t30576, t30582, t30589, t30590, t30592)
}
