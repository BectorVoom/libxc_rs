//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2065/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2065(t12995: f64, t26824: f64, t12963: f64, t7613: f64, t12975: f64, t2138: f64, t12984: f64, t12851: f64, t2134: f64, t3567: f64, t8945: f64, t26894: f64, t29199: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t97279 = t26824 * t12995;
    let t97281 = t7613 * t12963;
    let t97283 = t12975 * t2138;
    let t97288 = t7613 * t12984;
    let t97296 = 5.0_f64 / 1296.0_f64 * t2134 * t12851;
    let t97304 = t3567 * t8945;
    let t97308 = t26894 * t29199;
    (t97279, t97281, t97283, t97288, t97296, t97304, t97308)
}
