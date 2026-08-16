//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1345/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1345(t2832: f64, t892: f64, t2408: f64, t2411: f64, t3335: f64, t389: f64, t1077: f64, t225: f64, t1071: f64, t3046: f64, t268: f64, t271: f64, t7021: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11075 = t2832 * t892;
    let t11084 = t2408 * t2411;
    let t11108 = 1.0_f64 / t3335 / t389;
    let t11119 = t1077 * t1077;
    let t11120 = 1.0_f64 / t11119;
    let t11121 = t225 * t11120;
    let t11128 = t3046 * t1071;
    let t11132 = t268 * t7021 * t271;
    (t11075, t11084, t11108, t11121, t11128, t11132)
}
