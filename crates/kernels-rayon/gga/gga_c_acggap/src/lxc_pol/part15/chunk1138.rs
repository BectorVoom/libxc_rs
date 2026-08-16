//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1138/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1138(t1165: f64, t34278: f64, t5641: f64, t604: f64, t34368: f64, t34369: f64, t5693: f64, t34691: f64, t34692: f64, t5697: f64, t137: f64, t336: f64, t578: f64, t6119: f64) -> (f64, f64, f64, f64) {
    let t39690 = t34278 * t1165 * t604 * t5641;
    let t39693 = t34368 * t34369 * t5693;
    let t39696 = t34691 * t34692 * t5697;
    let t39700 = t578 * t336 * t6119 * t137;
    (t39690, t39693, t39696, t39700)
}
