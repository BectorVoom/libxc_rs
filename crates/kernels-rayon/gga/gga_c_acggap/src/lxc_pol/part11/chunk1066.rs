//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1066/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1066(t34632: f64, t1165: f64, t4521: f64, t7351: f64, t7426: f64, t2268: f64, t30797: f64, t30543: f64, t8473: f64, t4822: f64, t604: f64, t8463: f64) -> (f64, f64, f64, f64, f64) {
    let t34633 = 0.18868855373762491241e-1_f64 * t34632;
    let t34636 = t7426 * t1165 * t7351 * t4521;
    let t34638 = t30797 * t2268;
    let t34640 = t30543 * t8473;
    let t34644 = t8463 * t1165 * t604 * t4822;
    (t34633, t34636, t34638, t34640, t34644)
}
