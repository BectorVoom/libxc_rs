//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 610/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk610(t3398: f64, t3400: f64, t3409: f64, t3416: f64, t3419: f64, t3422: f64, t3425: f64, t3428: f64, t3432: f64, t3435: f64, t3441: f64, t3445: f64) -> f64 {
    let t3620 = -0.27801896084645508334e-2_f64 * t3398 + 0.10821235962619981449e-3_f64 * t3400 + 0.84410248952307505288e-7_f64 * t3409 + 0.16882049790461501058e-6_f64 * t3416 - 0.75883739738679928909e-7_f64 * t3419 + 0.1349212892553729136e-6_f64 * t3422 - 0.17376185052903442709e-3_f64 * t3425 - 0.17376185052903442709e-3_f64 * t3428 + 0.14480154210752868924e-5_f64 * t3432 - 0.2318836277704281739e-4_f64 * t3435 + 0.28136749650769168429e-8_f64 * t3441 - 0.69504740211613770835e-4_f64 * t3445;
    t3620
}
