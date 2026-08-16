//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1137/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1137(t1549: f64, t92968: f64, t2689: f64, t27239: f64, t14760: f64, t93015: f64, t1955: f64, t27198: f64, t2769: f64, t2453: f64, t27212: f64, t1568: f64, t786: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t99050 = t92968 * t1549;
    let t99091 = t2689 * t27239;
    let t99113 = t93015 * t14760;
    let t99191 = t1955 * t27198 * t2769;
    let t99257 = t2453 * t27212;
    let t99403 = t786 * t1568;
    (t99050, t99091, t99113, t99191, t99257, t99403)
}
