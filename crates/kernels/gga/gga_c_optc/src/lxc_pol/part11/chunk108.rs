//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 108/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk108<F: Float>(t214: F, t217: F, t220: F, t226: F) -> (F, F, F) {
    let t248 = F::cast_from(0.705945e1_f64) * t217 + F::cast_from(0.1549425e1_f64) * t214 + F::cast_from(0.420775e0_f64) * t220 + F::cast_from(0.1562925e0_f64) * t226;
    let t251 = F::cast_from(1.0_f64) + F::cast_from(0.32164683177870697974e2_f64) / t248;
    let t252 = F::ln(t251);
    (t248, t251, t252)
}
