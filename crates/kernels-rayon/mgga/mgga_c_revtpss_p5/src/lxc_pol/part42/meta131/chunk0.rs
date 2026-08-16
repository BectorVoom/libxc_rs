//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 629/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk629(t3478: f64, t3356: f64, t1175: f64, t1179: f64, t1178: f64, t444: f64) -> (f64, f64, f64, f64) {
    let t3479 = 1.0_f64 / t3478;
    let t3483 = 0.12361111111111111111e-1_f64 * t3356;
    let t3491 = t1175 * t1179;
    let t3494 = t1178 * t444;
    let t3495 = 1.0_f64 / t3494;
    (t3479, t3483, t3491, t3495)
}
