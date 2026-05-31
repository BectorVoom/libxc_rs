//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1275/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1275<F: Float>(t2848: F, t2854: F, t22035: F, t11: F, t8620: F) -> (F, F, F) {
    let t26255 = F::cast_from(1.0_f64) / t2848 / t2854;
    let t26256 = t26255 * t22035;
    let t26258 = t11 * t8620 * t26256;
    (t26255, t26256, t26258)
}
