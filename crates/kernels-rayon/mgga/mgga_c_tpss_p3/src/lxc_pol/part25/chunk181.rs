//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 181/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk181(t25: f64, t571: f64, t553: f64, t557: f64, t561: f64, t565: f64, t569: f64, t88: f64, t90: f64) -> (f64, f64, f64) {
    let t573 = 6.0_f64 * t25 * t571;
    let t574 = t553 - t557 + t561 - t565 + t569 - t573;
    let t577 = 1.0_f64 / t90 / t88;
    (t573, t574, t577)
}
