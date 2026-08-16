//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1999/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1999(t29551: f64, t7349: f64, t101870: f64, t101872: f64, t101874: f64, t101879: f64, t101881: f64, t108749: f64, t108759: f64, t109976: f64, t109980: f64, t109983: f64, t109985: f64, t109988: f64, t6960: f64, t7343: f64) -> f64 {
    let t109990 = t29551 * t7349;
    let t109992 = -5.0_f64 / 3.0_f64 * t7343 * t108749 + 10.0_f64 / 3.0_f64 * t109976 * t6960 - 4.0_f64 / 3.0_f64 * t109980 * t108759 - 8.0_f64 / 9.0_f64 * t109983 - 16.0_f64 / 9.0_f64 * t109985 - 8.0_f64 / 9.0_f64 * t109988 + 16.0_f64 / 9.0_f64 * t109990 + t101870 + t101872 + t101874 + t101879 + t101881;
    t109992
}
