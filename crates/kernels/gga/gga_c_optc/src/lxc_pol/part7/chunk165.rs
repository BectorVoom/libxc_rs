//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 165/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk165<F: Float>(t373: F, t376: F, t379: F, t383: F) -> (F, F, F) {
    let t398 = F::cast_from(0.705945e1_f64) * t376 + F::cast_from(0.1549425e1_f64) * t373 + F::cast_from(0.420775e0_f64) * t379 + F::cast_from(0.1562925e0_f64) * t383;
    let t401 = F::cast_from(1.0_f64) + F::cast_from(0.32164683177870697974e2_f64) / t398;
    let t402 = F::ln(t401);
    (t398, t401, t402)
}
