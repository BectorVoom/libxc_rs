//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 956/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk956(t11392: f64, t3159: f64, t10348: f64, t10485: f64, t2386: f64, t3338: f64, t544: f64, t6514: f64, t40549: f64, t40555: f64, t40558: f64, t40561: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t42444 = 0.25025342966295298669e1_f64 * t3159 * t11392;
    let t42448 = t10485 * t10348;
    let t42452 = t544 * t6514 * t3338 * t2386;
    let t42455 = 0.11916829983950142223e0_f64 * t40549;
    let t42456 = 0.89376224879626066674e-1_f64 * t40555;
    let t42457 = 0.59584149919750711116e-1_f64 * t40558;
    let t42458 = 0.59584149919750711116e-1_f64 * t40561;
    (t42444, t42448, t42452, t42455, t42456, t42457, t42458)
}
