//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1137/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1137(t7839: f64, t9641: f64, t1165: f64, t2068: f64, t604: f64, t6069: f64, t1181: f64, t6074: f64, t7351: f64, t7564: f64, t6351: f64, t7647: f64) -> (f64, f64, f64, f64) {
    let t39675 = t7839 * t9641;
    let t39679 = t2068 * t1165 * t604 * t6069;
    let t39683 = t7564 * t1181 * t7351 * t6074;
    let t39686 = t7647 * t6351;
    (t39675, t39679, t39683, t39686)
}
