//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1161/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1161(t10: f64, t6291: f64, t2034: f64, t6469: f64, t684: f64, t3150: f64, t689: f64, t2038: f64, t3023: f64, t704: f64, t2066: f64, t6012: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20241 = t6291 * t10;
    let t20252 = t684 * t6469 * t2034;
    let t20255 = t684 * t3150 * t689;
    let t20258 = t684 * t6469 * t2038;
    let t20278 = t3023 * t704;
    let t20280 = t6012 * t2066;
    (t20241, t20252, t20255, t20258, t20278, t20280)
}
