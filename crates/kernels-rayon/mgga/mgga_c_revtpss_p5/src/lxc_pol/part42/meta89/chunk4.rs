//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 524/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk524(t1312: f64, t2199: f64, t2201: f64, t651: f64, t3: f64, param_d: f64) -> (f64, f64, f64) {
    let t2204 = 2.0_f64 * t1312 * t2201 - 2.0_f64 * t2199 * t651;
    let t2205 = t3 * t2204;
    let t2207 = param_d * t2204;
    (t2204, t2205, t2207)
}
