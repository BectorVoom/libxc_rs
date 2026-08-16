//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 771/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk771(t1554: f64, t7822: f64, t1558: f64, t1421: f64, t599: f64, t1181: f64, t7493: f64, t1427: f64, t8463: f64, t1165: f64, t1432: f64, t7351: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8644 = t7822 * t1554;
    let t8646 = t7822 * t1558;
    let t8648 = t599 * t1421;
    let t8649 = t1181 * t8648;
    let t8650 = t7493 * t8649;
    let t8652 = t599 * t1427;
    let t8653 = t1181 * t8652;
    let t8654 = t8463 * t8653;
    let t8657 = t1165 * t7351 * t1432;
    (t8644, t8646, t8648, t8649, t8650, t8652, t8653, t8654, t8657)
}
