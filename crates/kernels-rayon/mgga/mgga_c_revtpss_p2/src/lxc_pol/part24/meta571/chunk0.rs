//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1749/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1749(t6470: f64, t1150: f64, t3384: f64, t3433: f64, t3435: f64, t1733: f64, t81146: f64, t20629: f64, t6471: f64, t6439: f64, t90293: f64, t90321: f64, t90323: f64, t90327: f64, t90329: f64, t90332: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t90333 = t6470 * t6470;
    let t90336 = 6.0_f64 * t3384 * t90333 * t1150;
    let t90339 = 0.48245938496077605201e2_f64 * t3433 * t90333 * t3435;
    let t90341 = 4.0_f64 * t81146 * t1733;
    let t90343 = 6.0_f64 * t20629 * t6471;
    let t90346 = 36.0_f64 * t3433 * t6439 * t6470;
    let t90347 = t90293 + t90321 - t90323 + t90327 + t90329 - t90332 - t90336 + t90339 + t90341 + t90343 + t90346;
    (t90336, t90339, t90341, t90343, t90346, t90347)
}
