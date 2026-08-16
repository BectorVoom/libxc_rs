//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1412/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1412(t5854: f64, t607: f64, t10355: f64, t5819: f64, t606: f64, t4186: f64, t4201: f64, t2275: f64, t5825: f64, t18281: f64, t48: f64, t10368: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21727 = t607 * t5854;
    let t21732 = t10355 * t5819;
    let t21733 = t21732 * t606;
    let t21736 = t4201 * t4186;
    let t21741 = t2275 * t5825;
    let t21742 = t21741 * t606;
    let t21745 = t48 * t18281;
    let t21754 = t10368 * t5819;
    (t21727, t21733, t21736, t21742, t21745, t21754)
}
