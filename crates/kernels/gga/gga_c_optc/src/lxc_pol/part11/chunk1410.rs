//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1410/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1410<F: Float>(t59160: F, t59162: F, t59165: F, t59169: F, t59171: F, t59173: F, t59176: F, t59179: F, t59181: F, t59183: F, t59186: F, t59188: F) -> F {
    let t59189 = t59160 + t59162 - t59165 - t59169 - t59171 - t59173 + t59176 + t59179 + t59181 + t59183 - t59186 + t59188;
    t59189
}
