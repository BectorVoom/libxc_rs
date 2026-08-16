//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1235/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1235(t5: f64, t1923: f64, t2123: f64, t7566: f64, t7702: f64, t7706: f64, t7709: f64, t8144: f64, t8147: f64, t117: f64) -> (f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t8151 = piecewise3(t8, 0.0_f64, -t7702 * t2123 / 6.0_f64 + 5.0_f64 / 6.0_f64 * t7566 * t7706 + t7709 * t2123 / 3.0_f64 - t1923 * t8144 / 6.0_f64 - t1923 * t8147 / 6.0_f64);
    let t8152 = t8151 * t117;
    (t8151, t8152)
}
