//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 972/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk972(t43522: f64, t2028: f64, t2033: f64, t42921: f64, t43462: f64, t43465: f64, t43468: f64, t43471: f64, t43477: f64, t43479: f64, t43481: f64, t43484: f64, t43489: f64, t43492: f64, t43497: f64, t43500: f64, t43502: f64, t43505: f64, t43511: f64, t43514: f64, t43516: f64, t43519: f64, t549: f64) -> f64 {
    let t43523 = 0.29792074959875355558e-1_f64 * t43522;
    let t43524 = 0.29792074959875355558e-1_f64 * t43462 + t43465 + t43468 + t43471 + 0.39722766613167140743e-1_f64 * t2033 * t549 * t42921 - t43477 - t43479 - 0.21450293971110256002e1_f64 * t43481 - 0.21450293971110256002e1_f64 * t43484 - t43489 - 0.18404604457881959845e2_f64 * t43492 - t43497 + t43500 + 0.29792074959875355558e-1_f64 * t43502 - 0.39722766613167140743e-1_f64 * t43505 * t2028 - t43511 + t43514 + 0.87421871174939309263e2_f64 * t43516 + 0.59584149919750711116e-1_f64 * t43519 + t43523;
    t43524
}
