//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 367/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk367(t50: f64, t296: f64, t34: f64, t1289: f64, t238: f64, t821: f64, t1288: f64, zeta_threshold: f64) -> (f64, f64) {
    let t51 = t50 <= zeta_threshold;
    let t1292 = t296 * t34;
    let t1296 = piecewise3(t51, 0.0_f64, -2.0_f64 / 9.0_f64 * t1289 * t238 - 4.0_f64 / 3.0_f64 * t1292 * t821);
    let t1298 = t1288 / 2.0_f64 + t1296 / 2.0_f64;
    (t1292, t1298)
}
