//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1206/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1206(t45: f64, t10446: f64, t1469: f64, t2375: f64, t4186: f64, t13312: f64, t2251: f64, t2258: f64, t4377: f64, t606: f64, t78: f64, t10457: f64, t2382: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t151 = t45 <= zeta_threshold;
    let t14401 = t10446 * t1469;
    let t14404 = t2375 * t4186;
    let t14412 = piecewise3(t151, 0.0_f64, -8.0_f64 / 27.0_f64 * t14401 * t2251 + 8.0_f64 / 9.0_f64 * t14404 * t606 + 4.0_f64 / 9.0_f64 * t4377 * t2258 + 4.0_f64 / 3.0_f64 * t78 * t13312);
    let t14413 = t10457 * t1469;
    let t14416 = t2382 * t4186;
    (t14412, t14413, t14416)
}
