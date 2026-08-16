//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 733/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk733(t3692: f64, t1389: f64, t219: f64, t1395: f64, t818: f64, t2406: f64, t2157: f64, t220: f64, t73: f64, t1378: f64, t246: f64, t768: f64, param_beta: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3693 = param_beta * t3692;
    let t3695 = t1389 * t219;
    let t3698 = t1395 * t818;
    let t3699 = t2406 * t3698;
    let t3703 = t220 * t73 * t2157;
    let t3704 = t246 * t1378;
    let t3713 = t220 * t73 * t768;
    (t3693, t3695, t3699, t3703, t3704, t3713)
}
