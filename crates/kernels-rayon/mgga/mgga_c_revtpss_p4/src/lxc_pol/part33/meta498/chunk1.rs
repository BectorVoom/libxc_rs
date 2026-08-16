//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1805/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1805(t10301: f64, t7565: f64, t38: f64, t7574: f64, t2247: f64, t2282: f64, t55: f64, t10309: f64) -> (f64, f64, f64, f64, f64) {
    let t26749 = t10301 * t7565;
    let t26754 = t38 * t7574;
    let t26755 = t2247 * t26754;
    let t26776 = t55 * t2282;
    let t26792 = t10309 * t7565;
    (t26749, t26754, t26755, t26776, t26792)
}
