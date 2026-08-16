//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 872/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk872(t3682: f64, t461: f64, t1226: f64, t140: f64, t1222: f64, t1225: f64, t2258: f64, t1012: f64, t1224: f64, t3367: f64, t2251: f64, t1121: f64, t404: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3684 = t461 * t3682 / 432.0_f64;
    let t3685 = t140 * t1226;
    let t3686 = t1222 * t3685;
    let t3688 = t1225 * t2258;
    let t3689 = t1012 * t3688;
    let t3692 = t1224 * t3367;
    let t3693 = t3692 * t2251;
    let t3694 = t1012 * t3693;
    let t3698 = 1.0_f64 / t404 / t1121;
    (t3684, t3685, t3686, t3688, t3689, t3693, t3694, t3698)
}
