//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1123/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1123(t5801: f64, t7822: f64, t1181: f64, t6226: f64, t7351: f64, t7564: f64, t1165: f64, t6198: f64, t8600: f64, t1784: f64, t2020: f64, t1095: f64, t1980: f64, t5659: f64, t7476: f64) -> (f64, f64, f64, f64, f64) {
    let t39414 = t7822 * t5801;
    let t39418 = t7564 * t1181 * t7351 * t6226;
    let t39422 = t7564 * t1165 * t8600 * t6198;
    let t39427 = t2020 * t1784;
    let t39438 = t1980 * t7476 * t1095 * t5659;
    (t39414, t39418, t39422, t39427, t39438)
}
