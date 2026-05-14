//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 883/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk883<F: Float>(t11269: F, t419: F, t85483: F, t37789: F, t420: F, t85469: F, t15741: F, t4431: F) -> (F, F, F) {
    let t85485 = t419 * t11269 * t85483;
    let t85489 = t419 * t420 * t37789 * t85469;
    let t85491 = t15741 * t4431;
    (t85485, t85489, t85491)
}
