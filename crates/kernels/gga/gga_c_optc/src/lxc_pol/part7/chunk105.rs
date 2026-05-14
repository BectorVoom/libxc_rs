//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 105/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk105<F: Float>(t214: F, t217: F, t220: F, t226: F) -> (F, F, F) {
    let t228 = 0.379785e1 * t217 + 0.8969e0 * t214 + 0.204775e0 * t220 + 0.123235e0 * t226;
    let t231 = 1.0 + 0.16081824322151104822e2 / t228;
    let t232 = f64::ln(t231);
    (t228, t231, t232)
}
