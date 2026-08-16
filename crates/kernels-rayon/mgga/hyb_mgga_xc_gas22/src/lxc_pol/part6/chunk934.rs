//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 934/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk934(t2014: f64, t3146: f64, t684: f64, t686: f64, t8184: f64, t3151: f64, t1238: f64, t6457: f64, t2033: f64, t3: f64, t3161: f64, t214: f64, t3271: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8476 = t684 * t2014 * t3146 / 96.0_f64;
    let t8477 = t8184 * t686;
    let t8479 = t684 * t8477 * t3151;
    let t8481 = t6457 * t1238;
    let t8485 = t2033 * t3;
    let t8491 = t684 * t2014 * t3161 / 96.0_f64;
    let t8492 = t3271 * t214;
    (t8476, t8477, t8479, t8481, t8485, t8491, t8492)
}
