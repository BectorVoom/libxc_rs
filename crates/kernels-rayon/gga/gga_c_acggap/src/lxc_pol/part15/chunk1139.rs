//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1139/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1139(t1181: f64, t39491: f64, t604: f64, t7493: f64, t1165: f64, t5693: f64, t7351: f64, t8463: f64, t1849: f64, t360: f64, t7575: f64, t6209: f64) -> (f64, f64, f64, f64) {
    let t39705 = t7493 * t1181 * t604 * t39491;
    let t39709 = t8463 * t1165 * t7351 * t5693;
    let t39720 = t7575 * t1181 * t7351 * t1849 * t360;
    let t39724 = t7575 * t1181 * t604 * t6209;
    (t39705, t39709, t39720, t39724)
}
