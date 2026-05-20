//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3452/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3452<F: Float>(t41361: F, t41363: F, t51973: F, t51978: F, t63325: F, t63328: F, t63336: F, t63338: F, t63340: F, t63342: F, t63346: F, t63351: F, t63355: F) -> F {
    let t64959 = -F::cast_from(0.26340740740740740742e-1_f64) * t51973 + F::cast_from(0.30730864197530864199e-1_f64) * t51978 + F::cast_from(0.30730864197530864198e-1_f64) * t41361 + F::cast_from(0.13170370370370370371e-1_f64) * t41363 - F::cast_from(0.65851851851851851853e-1_f64) * t63325 + F::cast_from(0.23706666666666666667e0_f64) * t63328 + F::cast_from(0.35560000000000000001e0_f64) * t63336 - F::cast_from(0.39511111111111111112e-1_f64) * t63338 + F::cast_from(0.13170370370370370371e-1_f64) * t63340 + F::cast_from(0.10975308641975308642e-1_f64) * t63342 - F::cast_from(0.16462962962962962963e-1_f64) * t63346 - F::cast_from(0.43901234567901234568e-1_f64) * t63351 + F::cast_from(0.59266666666666666668e-1_f64) * t63355;
    t64959
}
