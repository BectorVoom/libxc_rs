//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1125/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1125(t11007: f64, t8477: f64, t8648: f64, t1955: f64, t2681: f64, t8464: f64, t8468: f64, t596: f64, t31746: f64, t786: f64, t7063: f64, t31809: f64, t31837: f64) -> (f64, f64, f64, f64, f64) {
    let t120057 = t8477 * t8648 * t11007;
    let t120066 = t1955 * t8464 * t2681 * t8468;
    let t120068 = t8464 * t596;
    let t120070 = t786 * t120068 * t31746;
    let t120073 = t7063 * t120068 * t31746;
    let t120082 = t31809 * t31837;
    (t120057, t120066, t120070, t120073, t120082)
}
