//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1268/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1268(t3046: f64, t7135: f64, t1078: f64, t1982: f64, t3140: f64, t3259: f64, t378: f64, t42859: f64, t1032: f64, t7150: f64, t1071: f64, t11239: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t93459 = t3046 * t7135;
    let t93464 = t1982 * t3259 * t3140 * t1078;
    let t93469 = t378 * t42859;
    let t93471 = t1982 * t93469 * t1078;
    let t93484 = t3259 * t1032;
    let t93485 = t7150 * t93484;
    let t93488 = t1071 * t11239;
    (t93459, t93464, t93471, t93484, t93485, t93488)
}
