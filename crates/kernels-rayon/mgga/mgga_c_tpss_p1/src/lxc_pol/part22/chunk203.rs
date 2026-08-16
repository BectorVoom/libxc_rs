//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 203/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk203(t114: f64, t630: f64, t640: f64, t628: f64, t69: f64) -> (f64, f64) {
    let t115 = 1.0_f64 < t114;
    let t641 = t630 * t640;
    let t645 = piecewise3(t115, 0.0_f64, -t628 - t69 * t641 / 8.0_f64);
    (t641, t645)
}
