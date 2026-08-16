//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1937/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1937(t28042: f64, t508: f64, t651: f64, t1843: f64, t7002: f64, t2322: f64, t7742: f64, t4254: f64, t1310: f64, t7741: f64, t22496: f64, t8717: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t28043 = t508 * t28042;
    let t28045 = 2.0_f64 * t651 * t28043;
    let t28056 = t1843 * t7002;
    let t28058 = 2.0_f64 * t651 * t28056;
    let t28060 = 2.0_f64 * t2322 * t7742;
    let t28062 = 2.0_f64 * t4254 * t7742;
    let t28063 = t1310 * t7741;
    let t28065 = 2.0_f64 * t651 * t28063;
    let t28067 = t8717 * t22496;
    (t28043, t28045, t28056, t28058, t28060, t28062, t28063, t28065, t28067)
}
