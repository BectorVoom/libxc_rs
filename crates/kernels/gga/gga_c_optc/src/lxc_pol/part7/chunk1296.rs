//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1296/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1296<F: Float>(t56: F, t8950: F, t2848: F, t22035: F, t11: F) -> (F, F, F) {
    let t26334 = t56 * t8950;
    let t26335 = t2848 * t2848;
    let t26336 = F::new(1.0) / t26335;
    let t26337 = t26336 * t22035;
    let t26339 = t11 * t26334 * t26337;
    (t26336, t26337, t26339)
}
