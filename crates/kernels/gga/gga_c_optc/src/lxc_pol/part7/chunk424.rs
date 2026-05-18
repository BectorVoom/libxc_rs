//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 424/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk424<F: Float>(t127: F, t2067: F, t6: F, t161: F, t2023: F, t2040: F, t22: F) -> (F, F, F, F, F) {
    let t2069 = t6 * t2067 * t127;
    let t2070 = t161 * t2069;
    let t2073 = t2023 * t127;
    let t2074 = t161 * t2073;
    let t2078 = F::new(1.0) / t22 / t2040;
    (t2069, t2070, t2073, t2074, t2078)
}
