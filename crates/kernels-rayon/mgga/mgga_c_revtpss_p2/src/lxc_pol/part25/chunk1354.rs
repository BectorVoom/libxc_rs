//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1354/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1354(t1937: f64, t49856: f64, t18163: f64, t6993: f64, t25188: f64, t7239: f64, t46126: f64, t49851: f64, t10416: f64, t25081: f64, t7234: f64, t25083: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t95073 = 2.0_f64 * t49856 * t1937;
    let t95075 = 6.0_f64 * t18163 * t6993;
    let t95081 = 9.0_f64 * t25188 * t7239;
    let t95083 = 2.0_f64 * t46126 * t1937;
    let t95085 = 6.0_f64 * t49851 * t1937;
    let t95087 = 6.0_f64 * t10416 * t6993;
    let t95088 = t7234 * t25081;
    let t95090 = 18.0_f64 * t95088 * t25083;
    (t95073, t95075, t95081, t95083, t95085, t95087, t95090)
}
