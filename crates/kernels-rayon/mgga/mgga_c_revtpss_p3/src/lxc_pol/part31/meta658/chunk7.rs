//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2229/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2229(t21804: f64, t76: f64, t2242: f64, t5819: f64, t38: f64, t60670: f64, t1923: f64, t1926: f64, t1928: f64, t28078: f64, t28089: f64, t28093: f64, t29513: f64, t29532: f64, t29533: f64, t29551: f64, t6954: f64, t6973: f64, t6974: f64, t6978: f64, t7702: f64, t7715: f64, t7716: f64) -> f64 {
    let t108941 = t76 * t21804;
    let t108945 = t2242 * t5819;
    let t108952 = t60670 * t38;
    let t108963 = -t1923 * t7715 * t28089 / 3.0_f64 - t6954 * t29533 / 6.0_f64 - t1923 * t6973 * t29532 / 6.0_f64 - t1923 * t1926 * t108941 / 6.0_f64 + t108945 * t1928 / 3.0_f64 + t29551 * t6974 / 3.0_f64 + t29551 * t6978 / 3.0_f64 - t108952 * t1928 / 6.0_f64 - t29513 * t6974 / 6.0_f64 - t29513 * t6978 / 6.0_f64 - t28093 * t7716 / 3.0_f64 - t7702 * t28078 / 3.0_f64;
    t108963
}
