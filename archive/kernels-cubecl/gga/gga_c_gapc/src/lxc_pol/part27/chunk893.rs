//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 893/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk893<F: Float>(t10203: F, t2456: F, t3258: F, t3253: F, t6948: F, t10293: F, t6951: F, t3239: F, t6935: F, t2206: F, t761: F, t2920: F) -> (F, F, F, F, F, F) {
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
