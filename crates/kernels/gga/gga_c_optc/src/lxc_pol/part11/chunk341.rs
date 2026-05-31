//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 341/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk341<F: Float>(t1067: F, t1086: F, t1448: F, t1462: F, t1464: F, t1472: F, t1477: F, t1484: F, t241: F, t402: F, t1085: F, t1094: F, t1483: F) -> (F, F, F) {
    let t1488 = t241 * (-F::cast_from(0.3109e-1_f64) * t1464 * t402 + F::cast_from(1.0_f64) * t1067 * t1472 + t1448 - t1462 - F::cast_from(0.19751789702565206229e-1_f64) * t1477 + F::cast_from(0.58482233974552040708e0_f64) * t1086 * t1484);
    let t1490 = F::cast_from(0.19751789702565206229e-1_f64) * t241 * t1477;
    let t1492 = t1085 * t1483 * t1094;
    (t1488, t1490, t1492)
}
