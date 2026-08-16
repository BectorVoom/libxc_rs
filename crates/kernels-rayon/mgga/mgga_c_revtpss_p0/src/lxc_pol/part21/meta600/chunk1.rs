//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2326/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2326(t11028: f64, t2439: f64, t887: f64, t11021: f64, t2471: f64, t11024: f64, t689: f64, t2440: f64, t2772: f64, t10541: f64, t2453: f64, t10538: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39565 = t2439 * t11028 * t887;
    let t39567 = t11021 * t2471;
    let t39570 = t689 * t11024 * t887;
    let t39573 = t2439 * t2440 * t2772;
    let t39575 = t2453 * t10541;
    let t39576 = t39575 * t10538;
    (t39565, t39567, t39570, t39573, t39575, t39576)
}
