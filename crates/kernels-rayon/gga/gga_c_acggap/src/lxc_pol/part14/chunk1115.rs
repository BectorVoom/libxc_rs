//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1115/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1115(t1095: f64, t1980: f64, t5659: f64, t7476: f64, t1089: f64, t2079: f64, t2080: f64, t22099: f64, t1165: f64, t26995: f64, t604: f64, t7337: f64) -> (f64, f64, f64) {
    let t39438 = t1980 * t7476 * t1095 * t5659;
    let t39442 = t2079 * t1089 * t22099 * t2080;
    let t39447 = t7337 * t1165 * t604 * t26995;
    (t39438, t39442, t39447)
}
