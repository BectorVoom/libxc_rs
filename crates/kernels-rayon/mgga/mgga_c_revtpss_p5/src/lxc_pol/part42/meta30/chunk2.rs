//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 195/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk195(t20: f64, t588: f64, t12: f64, t19: f64, t2: f64, t27: f64, t21: f64, t579: f64) -> (f64, f64, f64, f64, f64) {
    let t590 = 4.0_f64 * t20 * t588;
    let t592 = t12 * t19 * t2;
    let t594 = 6.0_f64 * t592 * t27;
    let t595 = t21 * t579;
    let t596 = 1.0_f64 / t595;
    (t590, t592, t594, t595, t596)
}
