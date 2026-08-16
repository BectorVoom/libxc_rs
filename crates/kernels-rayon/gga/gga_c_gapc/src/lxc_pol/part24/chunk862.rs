//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 862/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk862(t10328: f64, t10329: f64, t3239: f64, t6935: f64, t3258: f64, t2206: f64, t761: f64, t2920: f64, t3227: f64, t297: f64, t493: f64, t7371: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10330 = t10328 * t10329;
    let t10332 = t3239 * t6935;
    let t10333 = t3258 * t10332;
    let t10335 = t761 * t2206;
    let t10336 = t2920 * t10335;
    let t10337 = t10336 * t3227;
    let t10339 = t493 * t297;
    let t10340 = t10339 * t7371;
    (t10330, t10333, t10335, t10336, t10337, t10340)
}
