//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2913/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2913(t3926: f64, t9909: f64, t9775: f64, t9981: f64, t1389: f64, t3964: f64, t40604: f64, t3961: f64, t9741: f64, t10111: f64, t22: f64, t4092: f64) -> (f64, f64, f64, f64, f64) {
    let t47304 = t9909 * t3926;
    let t47320 = t9775 * t9981;
    let t47337 = 0.11344944493805280483e-2_f64 * t3964 * t40604 * t1389;
    let t47338 = t9741 * t3961;
    let t47348 = t10111 * t4092 * t22;
    (t47304, t47320, t47337, t47338, t47348)
}
