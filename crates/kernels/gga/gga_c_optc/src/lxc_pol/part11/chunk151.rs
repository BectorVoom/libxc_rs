//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 151/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk151<F: Float>(t373: F, t376: F, t379: F, t383: F) -> (F, F, F) {
    let t385 = 0.379785e1 * t376 + 0.8969e0 * t373 + 0.204775e0 * t379 + 0.123235e0 * t383;
    let t388 = 1.0 + 0.16081824322151104822e2 / t385;
    let t389 = f64::ln(t388);
    (t385, t388, t389)
}
