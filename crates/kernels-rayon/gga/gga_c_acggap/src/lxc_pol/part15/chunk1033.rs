//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1033/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1033(t30120: f64, t8948: f64, t7839: f64, t8787: f64, t30689: f64, t5286: f64, t1181: f64, t22275: f64, t604: f64, t7493: f64, t1165: f64, t23745: f64, t7351: f64) -> (f64, f64, f64, f64, f64) {
    let t36162 = t30120 * t8948;
    let t36175 = t7839 * t8787;
    let t36177 = t30689 * t5286;
    let t36194 = t7493 * t1181 * t604 * t22275;
    let t36198 = t7493 * t1165 * t7351 * t23745;
    (t36162, t36175, t36177, t36194, t36198)
}
