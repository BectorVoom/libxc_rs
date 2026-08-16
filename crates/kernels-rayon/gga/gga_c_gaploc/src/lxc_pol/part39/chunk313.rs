//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 313/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk313(t2513: f64, t2515: f64, t2520: f64, t2522: f64, t471: f64, t64: f64, t931: f64) -> (f64, f64) {
    let t2524 = -21.0_f64 / 256.0_f64 * t2513 + 21.0_f64 / 8192.0_f64 * t2515 - 7.0_f64 / 8192.0_f64 * t2520 + 7.0_f64 / 256.0_f64 * t2522;
    let t2530 = t2524 * t471 - 4.0_f64 / 3.0_f64 * t931 * t64 - 7.0_f64 / 256.0_f64 * t2513 + 7.0_f64 / 768.0_f64 * t2522;
    (t2524, t2530)
}
