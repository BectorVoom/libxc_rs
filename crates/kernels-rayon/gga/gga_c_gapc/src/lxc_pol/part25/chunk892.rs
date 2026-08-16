//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 892/1444 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk892(t3206: f64, t763: f64, t462: f64, t762: f64, t3193: f64, t126: f64, t818: f64, t787: f64, t3187: f64, t297: f64, t3727: f64, t7371: f64) -> (f64, f64, f64, f64, f64) {
    let t10137 = t763 * t3206;
    let t10139 = t462 * t762;
    let t10140 = t10139 * t3193;
    let t10142 = t126 * t818;
    let t10143 = t10142 * t787;
    let t10144 = t3187 * t10143;
    let t10146 = t3727 * t297;
    let t10147 = t10146 * t7371;
    (t10137, t10140, t10142, t10144, t10147)
}
