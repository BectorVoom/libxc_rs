//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 1239/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk1239(t1030: f64, t27597: f64, t34026: f64, t21825: f64, t3680: f64, t35302: f64, t35304: f64, t35307: f64, t35309: f64, t35312: f64, t35316: f64, t35319: f64, t35323: f64, t35325: f64) -> f64 {
    let t35328 = t1030 * t34026 * t27597;
    let t35330 = t21825 * t3680;
    let t35332 = 0.10821235962619981449e-3_f64 * t35302 - 0.34416463048299153652e-7_f64 * t35304 - 0.8433973524305555556e-6_f64 * t35307 - 0.40483072916666666668e-4_f64 * t35309 + 0.24458523220486111112e-4_f64 * t35312 - 0.19323635647535681159e-7_f64 * t35316 + 0.74218967013888888891e-4_f64 * t35319 + 0.24599089445891203706e-6_f64 * t35323 + 0.30775559784820528656e-8_f64 * t35325 + 0.8976204937239320858e-9_f64 * t35328 + 0.10860115658064651693e-4_f64 * t35330;
    t35332
}
