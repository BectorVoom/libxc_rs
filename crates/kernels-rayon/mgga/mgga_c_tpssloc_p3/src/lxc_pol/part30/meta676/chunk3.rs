//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2110/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2110(t5445: f64, t641: f64, t72: f64, t19445: f64, t79: f64, t2240: f64, t27948: f64, t33: f64, t55921: f64, t6489: f64, t19299: f64, t608: f64) -> (f64, f64, f64, f64, f64) {
    let t96517 = t72 * t641 * t5445;
    let t96521 = t72 * t79 * t19445;
    let t96529 = t2240 * t33 * t27948;
    let t96532 = t55921 * t6489;
    let t96535 = t19299 * t608;
    (t96517, t96521, t96529, t96532, t96535)
}
