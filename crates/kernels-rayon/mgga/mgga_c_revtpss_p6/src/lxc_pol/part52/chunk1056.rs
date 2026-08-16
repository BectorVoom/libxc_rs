//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1056/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1056(t5: f64, t32597: f64, t8623: f64, t1925: f64, t84: f64, t640: f64, t8621: f64, t32151: f64, t32581: f64, t32584: f64, t32586: f64, t32590: f64, t32593: f64, t8620: f64) -> (f64, f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t32599 = 5.0_f64 / 27.0_f64 * t32597 * t8623;
    let t32600 = t84 * t1925;
    let t32602 = t8621 * t32600 * t640;
    let t32608 = piecewise3(t8, 0.0_f64, -5.0_f64 / 72.0_f64 * t32581 * t8623 + 5.0_f64 / 12.0_f64 * t32584 * t32586 + 5.0_f64 / 18.0_f64 * t32590 * t32593 + t32599 - 5.0_f64 / 36.0_f64 * t8620 * t32602 - 5.0_f64 / 72.0_f64 * t8620 * t32151);
    (t32599, t32600, t32602, t32608)
}
