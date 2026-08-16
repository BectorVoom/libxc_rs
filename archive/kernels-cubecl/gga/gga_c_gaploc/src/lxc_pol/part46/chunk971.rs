//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 971/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk971<F: Float>(t43522: F, t2028: F, t2033: F, t42921: F, t43462: F, t43465: F, t43468: F, t43471: F, t43477: F, t43479: F, t43481: F, t43484: F, t43489: F, t43492: F, t43497: F, t43500: F, t43502: F, t43505: F, t43511: F, t43514: F, t43516: F, t43519: F, t549: F) -> F {
    let t43523 = F::cast_from(0.29792074959875355558e-1_f64) * t43522;
    let t43524 = F::cast_from(0.29792074959875355558e-1_f64) * t43462 + t43465 + t43468 + t43471 + F::cast_from(0.39722766613167140743e-1_f64) * t2033 * t549 * t42921 - t43477 - t43479 - F::cast_from(0.21450293971110256002e1_f64) * t43481 - F::cast_from(0.21450293971110256002e1_f64) * t43484 - t43489 - F::cast_from(0.18404604457881959845e2_f64) * t43492 - t43497 + t43500 + F::cast_from(0.29792074959875355558e-1_f64) * t43502 - F::cast_from(0.39722766613167140743e-1_f64) * t43505 * t2028 - t43511 + t43514 + F::cast_from(0.87421871174939309263e2_f64) * t43516 + F::cast_from(0.59584149919750711116e-1_f64) * t43519 + t43523;
    t43524
}
