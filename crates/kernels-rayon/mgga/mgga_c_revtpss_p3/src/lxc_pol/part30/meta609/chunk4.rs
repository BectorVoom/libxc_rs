//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2079/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2079(t25901: f64, t97802: f64, t1364: f64, t27961: f64, t786: f64, t2453: f64, t3908: f64, t7911: f64, t136: f64, t2457: f64, t7920: f64, t94589: f64) -> (f64, f64, f64, f64, f64) {
    let t97804 = 0.14456046980341999104e-1_f64 * t97802 * t25901;
    let t97808 = 0.19514881078765566038e-1_f64 * t786 * t27961 * t1364;
    let t97810 = t2453 * t7911 * t3908;
    let t97814 = t7920 * t136 * t2457;
    let t97815 = t94589 * t97814;
    (t97804, t97808, t97810, t97814, t97815)
}
