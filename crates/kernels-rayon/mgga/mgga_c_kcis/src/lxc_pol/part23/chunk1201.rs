//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1201/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1201(t17311: f64, t27506: f64, t12338: f64, t28573: f64, t2253: f64, t52933: f64, t2069: f64, t27553: f64, t4189: f64, t5900: f64, t94816: f64, t39296: f64, t8186: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t97635 = 4.0_f64 * t17311 * t27506;
    let t97637 = 4.0_f64 * t12338 * t28573;
    let t97638 = t52933 * t2253;
    let t97641 = 2.0_f64 * t4189 * t27553 * t2069;
    let t97643 = 4.0_f64 * t94816 * t5900;
    let t97645 = 2.0_f64 * t39296 * t8186;
    (t97635, t97637, t97638, t97641, t97643, t97645)
}
