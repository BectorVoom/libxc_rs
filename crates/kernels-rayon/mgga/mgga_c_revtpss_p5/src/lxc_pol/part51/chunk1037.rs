//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1037/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1037(t2718: f64, t8471: f64, t119993: f64, t31779: f64, t1955: f64, t2681: f64, t8464: f64, t8468: f64, t596: f64, t31746: f64, t786: f64, t7063: f64) -> (f64, f64, f64, f64, f64) {
    let t120058 = t2718 * t8471;
    let t120063 = 0.19274729307122665472e-1_f64 * t31779 * t119993;
    let t120066 = t1955 * t8464 * t2681 * t8468;
    let t120067 = 0.74664478761315012733e-2_f64 * t120066;
    let t120068 = t8464 * t596;
    let t120070 = t786 * t120068 * t31746;
    let t120071 = 0.20077843028252776532e-3_f64 * t120070;
    let t120073 = t7063 * t120068 * t31746;
    (t120058, t120063, t120067, t120071, t120073)
}
