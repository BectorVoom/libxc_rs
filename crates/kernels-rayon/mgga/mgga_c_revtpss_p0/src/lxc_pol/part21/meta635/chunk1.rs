//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2407/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2407(t2467: f64, t41017: f64, t11043: f64, t2453: f64, t10506: f64, t11032: f64, t786: f64, t789: f64, t2458: f64, t2761: f64, t2444: f64, t2772: f64, t689: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41018 = t41017 * t2467;
    let t41020 = t2453 * t11043;
    let t41021 = t41020 * t10506;
    let t41026 = t786 * t11032 * t789;
    let t41029 = t2453 * t2761 * t2458;
    let t41032 = t689 * t2444 * t2772;
    (t41018, t41020, t41021, t41026, t41029, t41032)
}
