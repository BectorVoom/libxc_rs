//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 701/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk701<F: Float>(t10143: F, t10147: F, t10150: F, t10162: F, t10165: F, t10169: F, t10176: F, t10180: F, t9089: F, t9092: F, t9094: F, t9147: F, t9149: F, t9151: F) -> F {
    let t11998 = t10143 + t10147 + t10150 - t10162 + t10165 - t10169 - t9089 + t9092 - t9094 + t10176 - t10180 + t9147 - t9149 + t9151;
    t11998
}
