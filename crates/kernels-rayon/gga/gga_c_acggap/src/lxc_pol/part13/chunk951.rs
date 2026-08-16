//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 951/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk951(t151: f64, t2116: f64, t3668: f64, t409: f64, t1103: f64, t7746: f64, t7637: f64, t7709: f64, t2113: f64, t7610: f64, t2082: f64, t30567: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t31643 = t151 * t2116 * t3668;
    let t31644 = t31643 * t409;
    let t31646 = t7746 * t1103;
    let t31658 = t7637 * t7709;
    let t31660 = t7610 * t2113;
    let t31662 = t30567 * t2082;
    (t31643, t31644, t31646, t31658, t31660, t31662)
}
