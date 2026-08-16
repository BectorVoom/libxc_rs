//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 669/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk669(t3329: f64, t789: f64, t1336: f64, t2206: f64, t791: f64, t796: f64, t1342: f64, t238: f64, t801: f64) -> (f64, f64, f64, f64, f64) {
    let t3330 = t789 * t3329;
    let t3335 = t2206 * t1336;
    let t3336 = t3335 * t791;
    let t3338 = t796 * t3329;
    let t3342 = t238 * t801 * t1342;
    (t3330, t3335, t3336, t3338, t3342)
}
