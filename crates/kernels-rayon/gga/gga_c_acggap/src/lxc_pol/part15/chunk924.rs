//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 924/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk924(t3652: f64, t7741: f64, t3657: f64, t355: f64, t879: f64, t1095: f64, t7457: f64, t7458: f64, t2104: f64, t7780: f64, t2067: f64, t3073: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t31530 = t7741 * t3652;
    let t31532 = t7741 * t3657;
    let t31539 = t355 * t879;
    let t31542 = t7457 * t7458 * t1095 * t31539;
    let t31544 = t7780 * t2104;
    let t31562 = t3073 * t2067;
    (t31530, t31532, t31539, t31542, t31544, t31562)
}
