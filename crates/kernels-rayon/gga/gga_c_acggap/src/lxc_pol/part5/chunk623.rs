//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 623/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk623(t50: f64, t2910: f64, t478: f64, t34: f64, t829: f64, t1289: f64, t1292: f64, t296: f64, t39: f64, t4015: f64, t821: f64, t830: f64, t833: f64, zeta_threshold: f64) -> (f64, f64) {
    let t51 = t50 <= zeta_threshold;
    let t4084 = t2910 * t478;
    let t4087 = t829 * t34;
    let t4097 = piecewise3(t51, 0.0_f64, 8.0_f64 / 27.0_f64 * t4084 * t830 + 8.0_f64 / 9.0_f64 * t4087 * t4015 - 2.0_f64 / 9.0_f64 * t1289 * t833 - 4.0_f64 / 3.0_f64 * t296 * t821 + 4.0_f64 * t1292 * t39);
    (t4084, t4097)
}
