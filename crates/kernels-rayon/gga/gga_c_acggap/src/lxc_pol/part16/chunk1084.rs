//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1084/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1084(t7839: f64, t9758: f64, t2068: f64, t4680: f64, t9757: f64, t30268: f64, t9597: f64, t1181: f64, t25742: f64, t604: f64, t1165: f64, t26459: f64, t7337: f64) -> (f64, f64, f64, f64, f64) {
    let t39026 = t7839 * t9758;
    let t39029 = t2068 * t4680 * t9757;
    let t39031 = t30268 * t9597;
    let t39035 = t2068 * t1181 * t604 * t25742;
    let t39039 = t7337 * t1165 * t604 * t26459;
    (t39026, t39029, t39031, t39035, t39039)
}
