//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1361/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1361<F: Float>(t26336: F, t27082: F, t22035: F, t1114: F, t22046: F, t1111: F, t3097: F, t530: F, t26143: F, t27048: F, t27053: F, t27056: F, t27061: F, t27063: F, t27067: F, t27074: F, t27076: F, t27079: F, t3103: F, t3116: F, t3132: F, t322: F, t4357: F, t8469: F) -> (F, F, F) {
    let t27083 = t27082 * t26336;
    let t27084 = t27083 * t22035;
    let t27088 = t1114 * t22046;
    let t27093 = t1111 * t530 * t3097;
    let t27095 = -F::cast_from(0.18314556960919660338e2_f64) * t3132 * t27048 * t4357 - t27053 / F::cast_from(162.0_f64) - t27056 / F::cast_from(27.0_f64) - t27061 + F::cast_from(0.36629113921839320676e2_f64) * t3103 * t8469 * t27063 + F::cast_from(0.5680050638253047068e0_f64) * t3116 * t27067 * t26143 - t27074 + F::cast_from(5.0_f64) / F::cast_from(972.0_f64) * t27076 + F::cast_from(7.0_f64) / F::cast_from(486.0_f64) * t27079 + F::cast_from(35.0_f64) / F::cast_from(972.0_f64) * t1111 * t322 * t27084 + t1111 * t322 * t27088 / F::cast_from(288.0_f64) - t27093 / F::cast_from(216.0_f64);
    (t27084, t27088, t27095)
}
