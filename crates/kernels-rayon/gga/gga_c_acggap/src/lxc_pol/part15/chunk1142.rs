//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1142/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1142(t1967: f64, t9724: f64, t1988: f64, t9565: f64, t1089: f64, t3201: f64, t598: f64, t9563: f64, t1083: f64, t39219: f64, t1980: f64, t38893: f64, t7458: f64) -> (f64, f64, f64, f64, f64) {
    let t39767 = t1967 * t9724;
    let t39771 = t1988 * t9565;
    let t39775 = t598 * t1089 * t3201 * t9563;
    let t39779 = t598 * t1089 * t1083 * t39219;
    let t39782 = t1980 * t7458 * t38893;
    (t39767, t39771, t39775, t39779, t39782)
}
