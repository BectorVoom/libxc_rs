//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 713/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk713<F: Float>(t6359: F, t6437: F, t6625: F, t6627: F, t6634: F, t6638: F, t6640: F, t6644: F, t6647: F, t6694: F, t6696: F, t6709: F, t6737: F) -> F {
    let t6808 = t6625 - t6627 - t6634 - t6638 - t6640 - t6644 - t6647 - t6694 - t6696 - t6709 + t6359 + t6737 - t6437;
    t6808
}
