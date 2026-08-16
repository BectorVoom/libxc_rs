//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1106/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1106(t2030: f64, t361: f64, t9700: f64, t142: f64, t5506: f64, t599: f64, t2060: f64, t9704: f64, t1165: f64, t5969: f64, t604: f64, t7493: f64) -> (f64, f64, f64, f64) {
    let t39330 = t2030 * t361 * t9700;
    let t39334 = t2030 * t142 * t599 * t5506;
    let t39337 = t2060 * t361 * t9704;
    let t39343 = t7493 * t1165 * t604 * t5969;
    (t39330, t39334, t39337, t39343)
}
