//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1557/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1557(t1234: f64, t6594: f64, t1214: f64, t5825: f64, t5296: f64, t1042: f64, t3172: f64, t6630: f64, t3600: f64, t247: f64, t3634: f64, t6425: f64) -> (f64, f64, f64, f64) {
    let t21177 = t1234 * t6594;
    let t21182 = t5825 * t1214;
    let t21183 = t5296 * t21182;
    let t21184 = t1042 * t21183;
    let t21188 = t3172 * t6630;
    let t21189 = t3600 * t21188;
    let t21192 = t247 * t3634 * t6425;
    (t21177, t21184, t21189, t21192)
}
