//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1480/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1480(t1513: f64, t4287: f64, t5915: f64, t665: f64, t5920: f64, t648: f64, t21881: f64, t94: f64, t1518: f64, t4245: f64, t1501: f64, t4292: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t105875 = t1513 * t4287;
    let t105880 = t5915 * t665;
    let t108710 = t648 * t5920;
    let t108714 = t94 * t21881;
    let t109150 = t4245 * t1518;
    let t109153 = t1501 * t4292;
    (t105875, t105880, t108710, t108714, t109150, t109153)
}
