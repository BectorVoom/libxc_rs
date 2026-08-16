//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 893/1310 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk893(t10203: f64, t2456: f64, t3258: f64, t3253: f64, t6948: f64, t10293: f64, t6951: f64, t3239: f64, t6935: f64, t2206: f64, t761: f64, t2920: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10325 = t10203 * t2456;
    let t10326 = t3258 * t10325;
    let t10328 = t3253 * t6948;
    let t10329 = t10293 * t6951;
    let t10330 = t10328 * t10329;
    let t10332 = t3239 * t6935;
    let t10333 = t3258 * t10332;
    let t10335 = t761 * t2206;
    let t10336 = t2920 * t10335;
    (t10326, t10328, t10330, t10333, t10335, t10336)
}
