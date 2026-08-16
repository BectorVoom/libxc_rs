//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 979/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk979(t10643: f64, t10653: f64, t10660: f64, t11185: f64, t11188: f64, t11192: f64, t11195: f64, t11198: f64, t11202: f64, t11357: f64, t11369: f64, t11376: f64, t11468: f64) -> f64 {
    let t11470 = t11185 + 0.30487649791575028312e-3_f64 * t10643 - t11188 - t11192 + 0.1440846329149835838e-2_f64 * t10653 - t11195 - t11357 - 0.60975299583150056624e-3_f64 * t10660 - t11198 - t11202 + t11369 + t11376 + t11468;
    t11470
}
