//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1326/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1326(t12337: f64, t12335: f64, t12330: f64, t12347: f64, t12575: f64, t12435: f64, t37331: f64, t37332: f64, t37333: f64, t37334: f64, t37335: f64, t37336: f64, t37337: f64, t38519: f64, t38522: f64, t7: f64) -> f64 {
    let t38525 = 4.0_f64 * t12337;
    let t38526 = 4.0_f64 * t12335;
    let t38527 = 2.0_f64 * t12330;
    let t38528 = 4.0_f64 * t12347;
    let t38530 = 2.0_f64 * t12575;
    let tv4rho2sigma23 = t37331 - t37332 + t37333 - t37334 + t37335 - t37336 - t37337 + t7 * (t38519 + t38522) + t38525 - t38526 - t38527 + t38528 + 2.0_f64 * t12435 + t38530;
    tv4rho2sigma23
}
