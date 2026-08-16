//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1191/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1191(t30191: f64, t572: f64, t117: f64, t30004: f64, t1469: f64, t25137: f64, t26776: f64, t29355: f64, t5819: f64, t5825: f64, t5842: f64, t61: f64, t7571: f64) -> (f64, f64, f64, f64) {
    let t30193 = 6.0_f64 * t572 * t30191;
    let t30194 = t117 * t30004;
    let t30196 = 3.0_f64 * t572 * t30194;
    let t30681 = 88.0_f64 / 9.0_f64 * t5842 * t61 + 40.0_f64 / 9.0_f64 * t29355 * t1469 + 5.0_f64 / 18.0_f64 * t26776 * t5819 - 5.0_f64 / 6.0_f64 * t7571 * t5825 - t25137;
    (t30193, t30194, t30196, t30681)
}
