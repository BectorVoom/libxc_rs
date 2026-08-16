//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1905/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1905(t2435: f64, t8099: f64, t25904: f64, t26231: f64, t97802: f64, t26234: f64, t98041: f64, t102244: f64, t94674: f64, t97700: f64, t102268: f64, t102165: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t102315 = t8099 * t2435;
    let t102316 = t25904 * t102315;
    let t102320 = 0.14456046980341999104e-1_f64 * t97802 * t26231;
    let t102324 = 0.51405703062096148812e-1_f64 * t98041 * t26234;
    let t102325 = t94674 * t102244;
    let t102329 = 0.28912093960683998208e-1_f64 * t97700 * t26234;
    let t102339 = 0.14456046980341999104e-1_f64 * t25904 * t102268;
    let t102346 = 0.14456046980341999104e-1_f64 * t25904 * t102165;
    (t102315, t102316, t102320, t102324, t102325, t102329, t102339, t102346)
}
