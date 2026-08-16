//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1016/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1016(t357: f64, t988: f64, t378: f64, t42859: f64, t1071: f64, t11239: f64, t7150: f64, t3143: f64, t36870: f64, t11120: f64, t3140: f64, t25698: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t93437 = t357 * t988;
    let t93469 = t378 * t42859;
    let t93488 = t1071 * t11239;
    let t93962 = t7150 * t1071;
    let t93982 = t36870 * t3143;
    let t94014 = t3140 * t11120;
    let t94121 = t25698 * t378;
    (t93437, t93469, t93488, t93962, t93982, t94014, t94121)
}
