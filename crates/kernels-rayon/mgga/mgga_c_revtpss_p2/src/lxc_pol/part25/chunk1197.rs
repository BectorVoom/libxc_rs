//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1197/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1197(t25197: f64, t26092: f64, t3: f64, t2042: f64, t4158: f64, t1459: f64, t7331: f64, t7334: f64, t1936: f64, t2327: f64, t572: f64, t116: f64, t7002: f64, param_d: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t26093 = t25197 + t26092;
    let t26094 = t3 * t26093;
    let t26106 = param_d * t26093;
    let t26115 = 3.0_f64 * t4158 * t2042;
    let t26117 = 12.0_f64 * t1459 * t7331;
    let t26119 = 6.0_f64 * t1459 * t7334;
    let t26120 = t2327 * t1936;
    let t26122 = 6.0_f64 * t572 * t26120;
    let t26123 = t116 * t7002;
    (t26093, t26094, t26106, t26115, t26117, t26119, t26120, t26122, t26123)
}
