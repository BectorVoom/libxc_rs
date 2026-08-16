//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1209/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1209(t101252: f64, t101907: f64, t109911: f64, t109976: f64, t109980: f64, t110016: f64, t110018: f64, t110020: f64, t110022: f64, t114305: f64, t114311: f64, t114313: f64, t2048: f64, t29554: f64, t7343: f64, t7706: f64, t7964: f64) -> f64 {
    let t115324 = 40.0_f64 / 3.0_f64 * t110016 + 16.0_f64 / 3.0_f64 * t110018 + 32.0_f64 / 3.0_f64 * t110020 - 60.0_f64 * t101252 * t109911 - 80.0_f64 * t110022 + 88.0_f64 / 9.0_f64 * t101907 - 5.0_f64 / 3.0_f64 * t7343 * t114305 + 10.0_f64 * t109976 * t7706 - 2.0_f64 * t109980 * t114311 - 2.0_f64 / 3.0_f64 * t114313 * t2048 - 2.0_f64 * t29554 * t7964;
    t115324
}
