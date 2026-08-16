//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1319/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1319(t97156: f64, t97202: f64, t97249: f64, t97297: f64, t3567: f64, t8945: f64, t26894: f64, t29199: f64, t3596: f64, t37885: f64, t2149: f64, t1294: f64, t5464: f64) -> (f64, f64, f64, f64, f64) {
    let t97299 = t97156 + t97202 + t97249 + t97297;
    let t97304 = t3567 * t8945;
    let t97308 = t26894 * t29199;
    let t97312 = t37885 * t3596;
    let t97313 = t2149 * t97312;
    let t97314 = t5464 * t1294;
    (t97299, t97304, t97308, t97313, t97314)
}
