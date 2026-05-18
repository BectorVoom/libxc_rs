//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 844/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk844<F: Float>(t16257: F, t16318: F, t16319: F, t6359: F, t6437: F, t6449: F, t6457: F, t6638: F, t6644: F, t6696: F, t6709: F, t6741: F, t6747: F) -> F {
    let t16334 = -t16257 - t6638 - t6644 + t6696 - t6709 + t6359 + t16318 - t16319 - t6437 + t6449 + t6457 + t6741 - t6747;
    t16334
}
