//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 341/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk341<F: Float>(t1034: F, t1059: F, t1062: F, t1067: F, t1076: F, t1082: F, t1086: F, t1095: F, t241: F, t402: F, t406: F) -> (F, F, F) {
    let t1099 = t241 * (-F::new(0.3109e-1) * t1062 * t402 + F::new(1.0) * t1067 * t1076 + t1034 - t1059 - F::new(0.19751789702565206229e-1) * t1082 + F::new(0.58482233974552040708e0) * t1086 * t1095);
    let t1101 = F::new(0.19751789702565206229e-1) * t241 * t1082;
    let t1102 = t241 * t406;
    (t1099, t1101, t1102)
}
