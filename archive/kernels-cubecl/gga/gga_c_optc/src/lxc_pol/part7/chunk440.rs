//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 440/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk440<F: Float>(t106: F, t167: F, t2096: F, t2100: F, t2106: F, t2107: F, t2189: F, t670: F, t708: F) -> F {
    let t2193 = F::cast_from(0.27818116767324025134e1_f64) * t106 * t2096 * t167 - F::cast_from(0.55636233534648050268e1_f64) * t106 * t2100 * t708 + F::cast_from(0.55636233534648050268e1_f64) * t106 * t2106 * t2107 - F::cast_from(0.27818116767324025134e1_f64) * t106 * t670 * t2189;
    t2193
}
