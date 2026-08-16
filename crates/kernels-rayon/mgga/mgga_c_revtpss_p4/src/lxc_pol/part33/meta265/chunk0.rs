//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1180/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1180(t670: f64, t7330: f64, t572: f64, t117: f64, t7002: f64, t2121: f64, t38: f64) -> (f64, f64, f64, f64, f64) {
    let t7331 = t7330 * t670;
    let t7333 = 6.0_f64 * t572 * t7331;
    let t7334 = t117 * t7002;
    let t7336 = 3.0_f64 * t572 * t7334;
    let t7565 = t38 * t2121;
    (t7331, t7333, t7334, t7336, t7565)
}
