//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 940/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk940(t1390: f64, t5659: f64, t828: f64, t1883: f64, t221: f64, t4019: f64, t4018: f64, t241: f64, t4000: f64, t820: f64, t550: f64, t72: f64) -> (f64, f64, f64, f64, f64) {
    let t5661 = t1390 * t828 * t5659;
    let t5665 = t4019 * t221 * t1883;
    let t5666 = t4018 * t5665;
    let t5671 = t820 * t4000 * t241;
    let t5672 = t550 * t72;
    (t5661, t5665, t5666, t5671, t5672)
}
