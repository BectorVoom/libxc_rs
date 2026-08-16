//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 570/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk570(t43: f64, t1281: f64, t1284: f64, t292: f64, t39: f64, t4000: f64, t4070: f64, t4073: f64, t818: f64, t821: f64, t824: f64, t2910: f64, t478: f64, zeta_threshold: f64) -> (f64, f64) {
    let t44 = t43 <= zeta_threshold;
    let t4083 = piecewise3(t44, 0.0_f64, 8.0_f64 / 27.0_f64 * t4070 * t818 - 8.0_f64 / 9.0_f64 * t4073 * t4000 - 2.0_f64 / 9.0_f64 * t1281 * t824 + 4.0_f64 / 3.0_f64 * t292 * t821 - 4.0_f64 * t1284 * t39);
    let t4084 = t2910 * t478;
    (t4083, t4084)
}
