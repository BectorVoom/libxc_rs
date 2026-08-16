//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1032/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1032(t125: f64, t1444: f64, t246: f64, t551: f64, t32276: f64, t239: f64, t3999: f64, t8589: f64, t8583: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t32277 = t125 * t1444;
    let t32278 = t246 * t32277;
    let t32279 = t551 * t32278;
    let t32280 = t32276 * t32279;
    let t32282 = t3999 * t239;
    let t32283 = t8589 * t32282;
    let t32284 = t8583 * t32283;
    (t32278, t32279, t32280, t32282, t32283, t32284)
}
