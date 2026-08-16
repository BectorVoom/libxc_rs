//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1210/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1210(t57: f64, t4186: f64, t83: f64, t13312: f64, t1491: f64, t2251: f64, t2258: f64, t4335: f64, t606: f64, t770: f64, t14455: f64, t1568: f64, t785: f64, zeta_threshold: f64) -> (f64, f64) {
    let t155 = t57 <= zeta_threshold;
    let t14458 = t83 * t4186;
    let t14466 = piecewise3(t155, 0.0_f64, -8.0_f64 / 27.0_f64 * t1491 * t2251 - 4.0_f64 / 9.0_f64 * t14458 * t606 - 2.0_f64 / 9.0_f64 * t4335 * t2258 - 2.0_f64 / 3.0_f64 * t770 * t13312);
    let t14468 = t14455 / 2.0_f64 + t14466 / 2.0_f64;
    let t14472 = t785 * t1568;
    (t14468, t14472)
}
